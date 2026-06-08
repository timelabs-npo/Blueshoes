// Cloud Constitution §3: Spanner is a cross-edge constitutional memory fabric.
// It stores PROMOTED SEMANTIC KNOWLEDGE, not operational sovereignty.
// Data returned from Spanner is ADVISORY until locally validated.
// Edge must never block on Spanner availability.
// See: docs/CLOUD_CONSTITUTION.md

use super::GcpAuth;
use serde::{Deserialize, Serialize};

/// Data retrieved from the Spanner memory fabric.
///
/// Per CLOUD_CONSTITUTION.md §3.4, all Spanner data is treated as
/// **advisory** until the edge node independently validates it against
/// local lineage. The edge must never block on Spanner availability;
/// stale or unreachable Spanner = continue with last-known-good local state.
#[derive(Debug)]
pub struct AdvisoryResultSet {
    /// The raw rows returned. These are UNVALIDATED advisory data.
    /// Caller MUST validate before committing to local operational state.
    pub rows: Option<Vec<Vec<serde_json::Value>>>,
    pub field_names: Vec<String>,
    /// Whether the data was fetched live or fell back to a cached/empty state.
    pub source: AdvisorySource,
}

#[derive(Debug, PartialEq)]
pub enum AdvisorySource {
    /// Data was successfully fetched from Spanner (still advisory).
    LiveFetch,
    /// Spanner was unreachable; edge continues safely without it.
    Unavailable,
}

pub struct SpannerMemoryFabric {
    auth: GcpAuth,
    project_id: String,
    instance_id: String,
    database_id: String,
}

#[derive(Serialize)]
struct ExecuteSqlRequest {
    sql: String,
}

#[derive(Deserialize)]
struct RawResultSet {
    metadata: Option<RawResultSetMetadata>,
    rows: Option<Vec<Vec<serde_json::Value>>>,
}

#[derive(Deserialize)]
struct RawResultSetMetadata {
    #[serde(rename = "rowType")]
    row_type: Option<RawRowType>,
}

#[derive(Deserialize)]
struct RawRowType {
    fields: Option<Vec<RawField>>,
}

#[derive(Deserialize)]
struct RawField {
    name: String,
}

impl SpannerMemoryFabric {
    pub fn new(auth: GcpAuth, project_id: &str, instance_id: &str, database_id: &str) -> Self {
        Self {
            auth,
            project_id: project_id.to_string(),
            instance_id: instance_id.to_string(),
            database_id: database_id.to_string(),
        }
    }

    /// Query the cross-edge memory fabric for advisory data.
    ///
    /// # Constitutional Constraints (CLOUD_CONSTITUTION.md §3)
    ///
    /// - Returned data is **advisory**, not authoritative.
    /// - Caller must validate results against local lineage before acting.
    /// - This method **never panics** on network failure — it returns
    ///   `AdvisorySource::Unavailable` instead, preserving edge sovereignty.
    /// - This method must ONLY be used for READ queries (SELECT).
    ///   Write operations are restricted to the promotion pipeline.
    pub fn query_advisory(&self, sql: &str) -> AdvisoryResultSet {
        // Simple write operation guard: block INSERT or DELETE statements
        let sql_upper = sql.to_ascii_uppercase();
        if sql_upper.contains("INSERT") || sql_upper.contains("DELETE") {
            eprintln!("[spanner::memory_fabric] WRITE OPERATION BLOCKED by temporary shield.");
            return AdvisoryResultSet {
                rows: None,
                field_names: vec![],
                source: AdvisorySource::Unavailable,
            };
        }
        // Existing advisory read logic
        match self.execute_sql_inner(sql) {
            Ok(result) => result,
            Err(e) => {
                // CLOUD_CONSTITUTION §3.4: Stale or unreachable Spanner =
                // edge continues with last-known-good local state.
                eprintln!(
                    "[spanner::memory_fabric] Cloud unavailable (non-fatal): {}. \
                     Edge continues sovereign.",
                    e
                );
                AdvisoryResultSet {
                    rows: None,
                    field_names: vec![],
                    source: AdvisorySource::Unavailable,
                }
            }
        }
    }

    fn execute_sql_inner(
        &self,
        sql: &str,
    ) -> Result<AdvisoryResultSet, Box<dyn std::error::Error>> {
        let token = self.auth.get_token()?;

        let session_url = format!(
            "https://spanner.googleapis.com/v1/projects/{}/instances/{}/databases/{}/sessions",
            self.project_id, self.instance_id, self.database_id
        );

        // 1. Create a session
        let session_resp = ureq::post(&session_url)
            .set("Authorization", &format!("Bearer {}", token))
            .send_json(serde_json::json!({}))?;

        if session_resp.status() != 200 {
            return Err(
                format!("Failed to create Spanner session: {}", session_resp.status()).into(),
            );
        }

        let session_data: serde_json::Value = session_resp.into_json()?;
        let session_name = session_data["name"]
            .as_str()
            .ok_or("Missing session name")?;

        // 2. Execute SQL (read-only)
        let execute_url = format!(
            "https://spanner.googleapis.com/v1/{}:executeSql",
            session_name
        );
        let req_body = ExecuteSqlRequest {
            sql: sql.to_string(),
        };

        let exec_resp = ureq::post(&execute_url)
            .set("Authorization", &format!("Bearer {}", token))
            .set("Content-Type", "application/json")
            .send_json(req_body)?;

        if exec_resp.status() != 200 {
            return Err(format!("Failed to execute SQL: {}", exec_resp.status()).into());
        }

        let raw: RawResultSet = exec_resp.into_json()?;

        let field_names = raw
            .metadata
            .and_then(|m| m.row_type)
            .and_then(|rt| rt.fields)
            .map(|fields| fields.into_iter().map(|f| f.name).collect())
            .unwrap_or_default();

        Ok(AdvisoryResultSet {
            rows: raw.rows,
            field_names,
            source: AdvisorySource::LiveFetch,
        })
    }
}
