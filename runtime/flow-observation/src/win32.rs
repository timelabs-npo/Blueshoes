//! The only unsafe production boundary: query-only Windows APIs, no setters.
use crate::{adapters::*, model::*};
use std::{
    collections::{BTreeMap, BTreeSet},
    mem::{offset_of, size_of},
    net::{Ipv4Addr, Ipv6Addr},
    ptr,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};
use windows_sys::Win32::{
    Foundation::{CloseHandle, ERROR_INSUFFICIENT_BUFFER, FILETIME, HANDLE},
    NetworkManagement::IpHelper::{
        GetExtendedTcpTable, MIB_TCP6ROW_OWNER_PID, MIB_TCP6TABLE_OWNER_PID, MIB_TCPROW_OWNER_PID,
        MIB_TCPTABLE_OWNER_PID, TCP_TABLE_OWNER_PID_ALL,
    },
    Networking::WinSock::{AF_INET, AF_INET6},
    System::Threading::{GetProcessTimes, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION},
};

const MAX_TABLE_BYTES: usize = 16 * 1024 * 1024;
const MAX_ATTEMPTS: usize = 3;
const LISTEN: u32 = 2;
static SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct ProcessHandle(HANDLE);
impl Drop for ProcessHandle {
    fn drop(&mut self) {
        // SAFETY: created by a successful OpenProcess and owned exactly once.
        unsafe {
            CloseHandle(self.0);
        }
    }
}

fn process_birth(pid: u32) -> Option<u64> {
    if pid == 0 {
        return None;
    }
    // SAFETY: fixed limited query right; no inheritance or process mutation access.
    let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) };
    if handle.is_null() {
        return None;
    }
    let handle = ProcessHandle(handle);
    let mut birth = FILETIME::default();
    let mut exit = FILETIME::default();
    let mut kernel = FILETIME::default();
    let mut user = FILETIME::default();
    // SAFETY: live owned handle and four initialized, writable FILETIME objects.
    if unsafe { GetProcessTimes(handle.0, &mut birth, &mut exit, &mut kernel, &mut user) } == 0 {
        return None;
    }
    let stamp = (u64::from(birth.dwHighDateTime) << 32) | u64::from(birth.dwLowDateTime);
    (stamp != 0).then_some(stamp)
}

/// Owns a DWORD-aligned buffer. All pointer reads are checked against used bytes.
struct TableBuffer {
    words: Vec<u32>,
    used: usize,
}
impl TableBuffer {
    fn row_count(&self, offset: usize, row_size: usize) -> FlowResult<usize> {
        if self.used < size_of::<u32>() || self.used > self.words.len() * size_of::<u32>() {
            return Err("invalid TCP table header size".into());
        }
        let count = self.words[0] as usize;
        let end = count
            .checked_mul(row_size)
            .and_then(|n| offset.checked_add(n))
            .ok_or("TCP table size overflow")?;
        if count > MAX_RECORDS || end > self.used {
            return Err("invalid TCP table row count".into());
        }
        Ok(count)
    }
}

fn table(family: u32) -> FlowResult<TableBuffer> {
    let mut size = 0;
    // SAFETY: documented null buffer size query; fixed owner-PID query class.
    let status = unsafe {
        GetExtendedTcpTable(
            ptr::null_mut(),
            &mut size,
            0,
            family,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        )
    };
    if status != ERROR_INSUFFICIENT_BUFFER && status != 0 {
        return Err(format!("TCP table size query failed: {status}"));
    }
    for _ in 0..MAX_ATTEMPTS {
        if !(size_of::<u32>()..=MAX_TABLE_BYTES).contains(&(size as usize)) {
            return Err("TCP table allocation limit".into());
        }
        let mut words = vec![0u32; (size as usize).div_ceil(size_of::<u32>())];
        size = (words.len() * size_of::<u32>()) as u32;
        // SAFETY: allocated DWORD-aligned buffer of exactly advertised capacity.
        // The API may update size; it never permits us to read beyond our allocation.
        let status = unsafe {
            GetExtendedTcpTable(
                words.as_mut_ptr().cast(),
                &mut size,
                0,
                family,
                TCP_TABLE_OWNER_PID_ALL,
                0,
            )
        };
        if status == 0 {
            if size as usize > words.len() * size_of::<u32>() {
                return Err("TCP table exceeds allocation".into());
            }
            return Ok(TableBuffer {
                words,
                used: size as usize,
            });
        }
        if status != ERROR_INSUFFICIENT_BUFFER {
            return Err(format!("TCP table query failed: {status}"));
        }
    }
    Err("TCP table changed across bounded resize retries".into())
}

#[derive(Clone)]
struct SocketRow {
    pid: u32,
    source: Endpoint,
    destination: Endpoint,
}

fn unknown_endpoint() -> Endpoint {
    Endpoint {
        address_state: AddressState::Unknown,
        address: None,
        port: None,
        domain: None,
    }
}
fn endpoint(address: String, raw_port: u32, unspecified: bool) -> Endpoint {
    Endpoint {
        address_state: if unspecified {
            AddressState::Partial
        } else {
            AddressState::Known
        },
        address: Some(address),
        port: Some(u16::from_be(raw_port as u16)),
        domain: None,
    }
}
fn ipv6_address(bytes: [u8; 16], raw_scope: u32) -> String {
    let address = Ipv6Addr::from(bytes);
    let scope = u32::from_be(raw_scope);
    if scope == 0 {
        address.to_string()
    } else {
        format!("{address}%{scope}")
    }
}
fn ipv4_row(row: MIB_TCPROW_OWNER_PID) -> SocketRow {
    let local = Ipv4Addr::from(row.dwLocalAddr.to_ne_bytes());
    let remote = Ipv4Addr::from(row.dwRemoteAddr.to_ne_bytes());
    SocketRow {
        pid: row.dwOwningPid,
        source: endpoint(local.to_string(), row.dwLocalPort, local.is_unspecified()),
        destination: if row.dwState == LISTEN {
            unknown_endpoint()
        } else {
            endpoint(
                remote.to_string(),
                row.dwRemotePort,
                remote.is_unspecified(),
            )
        },
    }
}
fn ipv6_row(row: MIB_TCP6ROW_OWNER_PID) -> SocketRow {
    SocketRow {
        pid: row.dwOwningPid,
        source: endpoint(
            ipv6_address(row.ucLocalAddr, row.dwLocalScopeId),
            row.dwLocalPort,
            row.ucLocalAddr == [0; 16],
        ),
        destination: if row.dwState == LISTEN {
            unknown_endpoint()
        } else {
            endpoint(
                ipv6_address(row.ucRemoteAddr, row.dwRemoteScopeId),
                row.dwRemotePort,
                row.ucRemoteAddr == [0; 16],
            )
        },
    }
}

fn socket_rows() -> FlowResult<Vec<SocketRow>> {
    let v4 = table(u32::from(AF_INET))?;
    let offset4 = offset_of!(MIB_TCPTABLE_OWNER_PID, table);
    let count4 = v4.row_count(offset4, size_of::<MIB_TCPROW_OWNER_PID>())?;
    let v6 = table(u32::from(AF_INET6))?;
    let offset6 = offset_of!(MIB_TCP6TABLE_OWNER_PID, table);
    let count6 = v6.row_count(offset6, size_of::<MIB_TCP6ROW_OWNER_PID>())?;
    if count4 + count6 > MAX_RECORDS {
        return Err("combined TCP row limit".into());
    }
    let mut rows = Vec::with_capacity(count4 + count6);
    for i in 0..count4 {
        // SAFETY: row_count verified every row's complete byte extent; read_unaligned
        // copies a C struct containing only integer fields, so all bit patterns are valid.
        let row = unsafe {
            v4.words
                .as_ptr()
                .cast::<u8>()
                .add(offset4 + i * size_of::<MIB_TCPROW_OWNER_PID>())
                .cast::<MIB_TCPROW_OWNER_PID>()
                .read_unaligned()
        };
        rows.push(ipv4_row(row));
    }
    for i in 0..count6 {
        // SAFETY: same extent/bit-pattern checks as the IPv4 table above.
        let row = unsafe {
            v6.words
                .as_ptr()
                .cast::<u8>()
                .add(offset6 + i * size_of::<MIB_TCP6ROW_OWNER_PID>())
                .cast::<MIB_TCP6ROW_OWNER_PID>()
                .read_unaligned()
        };
        rows.push(ipv6_row(row));
    }
    Ok(rows)
}

// Private seam for deterministic query-order/error tests. It cannot accept an executor
// through the public observation API. Production has exactly one implementation.
trait Query {
    fn rows(&mut self) -> FlowResult<Vec<SocketRow>>;
    fn birth(&mut self, pid: u32) -> Option<u64>;
}
struct WindowsQuery;
impl Query for WindowsQuery {
    fn rows(&mut self) -> FlowResult<Vec<SocketRow>> {
        socket_rows()
    }
    fn birth(&mut self, pid: u32) -> Option<u64> {
        process_birth(pid)
    }
}
fn now() -> String {
    chrono::DateTime::<chrono::Utc>::from(SystemTime::now()).to_rfc3339()
}

fn collect_with(query: &mut impl Query) -> FlowResult<ObservationBatch> {
    let discovery = query.rows()?;
    if discovery.len() > MAX_RECORDS {
        return Err("discovery row limit".into());
    }
    let before: BTreeMap<_, _> = discovery
        .iter()
        .map(|r| r.pid)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|pid| (pid, query.birth(pid)))
        .collect();
    let started_at = now();
    let rows = query.rows()?; // Authoritative sample, AFTER first process identity reads.
    if rows.len() > MAX_RECORDS {
        return Err("snapshot row limit".into());
    }
    let after: BTreeMap<_, _> = rows
        .iter()
        .map(|r| r.pid)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|pid| (pid, query.birth(pid)))
        .collect(); // Fresh OpenProcess per PID.
    let finished_at = now();
    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "system clock predates epoch")?
        .as_nanos();
    let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let sample = NativeSample {
        schema_version: NativeVersion::NativeFlowSampleV1,
        platform: Platform::Win32,
        collector_scope: format!("win32-{}-{epoch}-{sequence}", std::process::id()),
        snapshot_id: format!("sample-{sequence}"),
        observed_at: started_at.clone(),
        freshness: Freshness::Fresh,
        records: rows
            .into_iter()
            .enumerate()
            .map(|(index, row)| NativeRecord {
                record_id: index.to_string(),
                process: Some(ProcessEvidence {
                    pid: row.pid,
                    kind: BirthKind::Win32Creation100ns,
                    birth_before: before.get(&row.pid).copied().flatten(),
                    birth_after: after.get(&row.pid).copied().flatten(),
                }),
                source: row.source,
                destination: row.destination,
                protocol: "tcp".into(),
                counters: Counters {
                    bytes_up: None,
                    bytes_down: None,
                    rate_up_bps: None,
                    rate_down_bps: None,
                },
                policy: Policy {
                    match_state: MatchState::Unknown,
                    rule_ref: None,
                    decision: None,
                },
                route: Route {
                    state: RouteState::Unknown,
                    interface_ref: None,
                    egress_ref: None,
                    tunnel_ref: None,
                },
                display_location: None,
            })
            .collect(),
    };
    let evaluation = Evaluation::new(&finished_at, 30_000)?;
    Ok(ObservationBatch {
        platform: Platform::Win32,
        native_gate: Gate::Pass,
        gaps: vec![
            "UDP, counters, rates, route and policy collection NOT_EXECUTED".into(),
            "Process binding is bracketed evidence; atomic socket lifetime/continuity NOT_EXECUTED"
                .into(),
        ],
        observations: normalize(sample, &evaluation, Origin::Native)?,
        collection_interval: Some(CollectionInterval {
            started_at,
            finished_at,
        }),
    })
}

pub(crate) fn collect() -> FlowResult<ObservationBatch> {
    collect_with(&mut WindowsQuery)
}

#[cfg(test)]
mod tests {
    use super::*;
    struct FakeQuery {
        phase: u8,
        before: Option<u64>,
        after: Option<u64>,
        fail_sample: bool,
        new_pid: bool,
        calls: Vec<String>,
    }
    impl Query for FakeQuery {
        fn rows(&mut self) -> FlowResult<Vec<SocketRow>> {
            self.phase += 1;
            self.calls.push(format!("rows-{}", self.phase));
            if self.fail_sample && self.phase == 2 {
                return Err("injected query failure".into());
            }
            Ok(vec![SocketRow {
                pid: if self.new_pid && self.phase == 2 {
                    2
                } else {
                    1
                },
                source: unknown_endpoint(),
                destination: unknown_endpoint(),
            }])
        }
        fn birth(&mut self, pid: u32) -> Option<u64> {
            self.calls.push(format!("birth-{pid}-{}", self.phase));
            if self.phase == 1 {
                self.before
            } else {
                self.after
            }
        }
    }
    fn fake(before: Option<u64>, after: Option<u64>) -> FakeQuery {
        FakeQuery {
            phase: 0,
            before,
            after,
            fail_sample: false,
            new_pid: false,
            calls: vec![],
        }
    }
    #[test]
    fn identity_queries_bracket_authoritative_snapshot() {
        let mut query = fake(Some(123), Some(123));
        let batch = collect_with(&mut query).unwrap();
        assert!(batch.observations[0].observation().process_ref.is_some());
        assert_eq!(query.calls, ["rows-1", "birth-1-1", "rows-2", "birth-1-2"]);
    }
    #[test]
    fn missing_rebound_or_new_pid_cannot_bind() {
        for (before, after) in [
            (None, Some(1)),
            (Some(1), None),
            (Some(1), Some(2)),
            (Some(0), Some(0)),
        ] {
            let batch = collect_with(&mut fake(before, after)).unwrap();
            assert!(batch.observations[0].observation().process_ref.is_none());
        }
        let mut query = fake(Some(1), Some(1));
        query.new_pid = true;
        assert!(collect_with(&mut query).unwrap().observations[0]
            .observation()
            .process_ref
            .is_none());
    }
    #[test]
    fn query_failure_is_error_not_successful_empty_batch() {
        let mut query = fake(Some(1), Some(1));
        query.fail_sample = true;
        assert!(collect_with(&mut query).is_err());
    }
    #[test]
    fn tcp_bytes_and_listener_peer_are_not_fabricated() {
        let row = ipv4_row(MIB_TCPROW_OWNER_PID {
            dwState: LISTEN,
            dwLocalAddr: u32::from_ne_bytes([127, 0, 0, 1]),
            dwLocalPort: u32::from(443u16.to_be()),
            dwRemoteAddr: 123,
            dwRemotePort: 123,
            dwOwningPid: 1,
        });
        assert_eq!(row.source.address.as_deref(), Some("127.0.0.1"));
        assert_eq!(row.source.port, Some(443));
        assert_eq!(row.destination, unknown_endpoint());
        let batch = collect_with(&mut fake(Some(1), Some(1))).unwrap();
        assert_eq!(batch.observations[0].observation().counters.bytes_up, None);
        assert_eq!(
            batch.observations[0].observation().counters.bytes_down,
            None
        );
    }
    #[test]
    fn ipv6_scopes_and_port_byte_order_survive() {
        let row = ipv6_row(MIB_TCP6ROW_OWNER_PID {
            ucLocalAddr: "fe80::1".parse::<Ipv6Addr>().unwrap().octets(),
            dwLocalScopeId: 3u32.to_be(),
            dwLocalPort: u32::from(50000u16.to_be()),
            ucRemoteAddr: "fe80::2".parse::<Ipv6Addr>().unwrap().octets(),
            dwRemoteScopeId: 4u32.to_be(),
            dwRemotePort: u32::from(443u16.to_be()),
            dwState: 5,
            dwOwningPid: 1,
        });
        assert_eq!(row.source.address.as_deref(), Some("fe80::1%3"));
        assert_eq!(row.source.port, Some(50000));
        assert_eq!(row.destination.address.as_deref(), Some("fe80::2%4"));
        assert_eq!(row.destination.port, Some(443));
    }
    #[test]
    fn corrupt_table_sizes_reject_before_pointer_read() {
        for buffer in [
            TableBuffer {
                words: vec![1],
                used: 4,
            },
            TableBuffer {
                words: vec![u32::MAX],
                used: 4,
            },
            TableBuffer {
                words: vec![],
                used: 0,
            },
            TableBuffer {
                words: vec![0],
                used: 8,
            },
        ] {
            assert!(buffer
                .row_count(4, size_of::<MIB_TCPROW_OWNER_PID>())
                .is_err());
        }
        assert_eq!(
            TableBuffer {
                words: vec![0],
                used: 4
            }
            .row_count(4, size_of::<MIB_TCPROW_OWNER_PID>())
            .unwrap(),
            0
        );
    }
}
