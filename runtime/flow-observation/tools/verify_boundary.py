"""Bounded build evidence, not a universal safety proof or a source keyword scan.

Run after cargo build --release. Tests invoke only the built observation CLI and
build metadata tools. No integration with the existing executor is imported.
"""
import hashlib
import json
import os
from pathlib import Path
import struct
import subprocess
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
REPO = ROOT.parents[1]
BINARY = ROOT / 'target/release' / ('bs-flow-observation.exe' if os.name == 'nt' else 'bs-flow-observation')
ALLOWED_PACKAGES = {'bs-flow-observation', 'chrono', 'num-traits', 'autocfg', 'serde',
    'serde_core', 'serde_derive', 'proc-macro2', 'quote', 'syn', 'unicode-ident',
    'serde_json', 'itoa', 'memchr', 'zmij', 'windows-sys', 'windows-link'}


def execute(args):
    return subprocess.run(args, cwd=ROOT, capture_output=True, text=True, check=False, timeout=60)


def pe_imports(path):
    """Parse the linked PE import directory rather than searching symbol text."""
    data = path.read_bytes()
    def word(offset, fmt):
        return struct.unpack_from(fmt, data, offset)[0]
    def cstring(offset):
        return data[offset:data.index(b'\0', offset)].decode('ascii')
    assert data[:2] == b'MZ'
    pe = word(0x3c, '<I')
    assert data[pe:pe+4] == b'PE\0\0'
    count = word(pe+6, '<H')
    optional_size = word(pe+20, '<H')
    optional = pe+24
    magic = word(optional, '<H')
    assert magic in (0x20b, 0x10b)
    width = 8 if magic == 0x20b else 4
    directory = optional + (112 if width == 8 else 96)
    sections = optional + optional_size
    def offset(rva):
        if rva < word(optional+60, '<I'):
            return rva
        for index in range(count):
            section = sections + index*40
            va = word(section+12, '<I')
            raw_size = word(section+16, '<I')
            if va <= rva < va + raw_size:
                result = word(section+20, '<I') + rva - va
                assert result < len(data)
                return result
        raise ValueError('unmapped PE RVA')
    descriptor = offset(word(directory+8, '<I'))
    imports = {}
    for _ in range(256):
        original, _, _, name, first = struct.unpack_from('<IIIII', data, descriptor)
        if not any((original, name, first)):
            return {dll: sorted(names) for dll, names in sorted(imports.items())}
        dll = cstring(offset(name)).lower()
        names = imports.setdefault(dll, set())
        thunk = offset(original or first)
        for i in range(4096):
            value = word(thunk+i*width, '<Q' if width == 8 else '<I')
            if not value:
                break
            if value & (1 << (width*8-1)):
                names.add('#' + str(value & 0xffff))
            else:
                names.add(cstring(offset(value)+2))
        else:
            raise ValueError('unbounded PE thunk table')
        descriptor += 20
    raise ValueError('unbounded PE import directory')


class BoundaryTests(unittest.TestCase):
    def test_production_dependency_closure_has_no_executor_or_network_client(self):
        version = execute(['rustc', '-vV'])
        self.assertEqual(0, version.returncode, version.stderr)
        target = next(line[6:] for line in version.stdout.splitlines() if line.startswith('host: '))
        result = execute(['cargo', 'metadata', '--locked', '--offline', '--format-version', '1', '--filter-platform', target])
        self.assertEqual(0, result.returncode, result.stderr)
        metadata = json.loads(result.stdout)
        packages = {p['id']: p for p in metadata['packages']}
        nodes = {n['id']: n for n in metadata['resolve']['nodes']}
        pending, visited = [metadata['resolve']['root']], set()
        while pending:
            current = pending.pop()
            if current in visited:
                continue
            visited.add(current)
            package = packages[current]
            self.assertIn(package['name'], ALLOWED_PACKAGES)
            if package['name'] != 'bs-flow-observation':
                self.assertTrue(package['source'].startswith('registry+'), package['name'])
            else:
                self.assertFalse(any('custom-build' in t['kind'] for t in package['targets']))
            pending.extend(d['pkg'] for d in nodes[current]['deps'] if any(k['kind'] != 'dev' for k in d['dep_kinds']))
        print('Production dependency closure:', ', '.join(sorted(packages[p]['name'] for p in visited)))

    def test_reviewed_production_source_set_is_unchanged(self):
        lock = json.loads((ROOT/'reviewed-source-lock.json').read_text())
        actual = {p.relative_to(ROOT).as_posix() for p in (ROOT/'src').rglob('*') if p.is_file()}
        self.assertEqual(actual | {'Cargo.toml', 'Cargo.lock'}, set(lock['files']))
        for name, expected in lock['files'].items():
            # Repository source fingerprints use LF bytes independently of Git checkout conversion.
            content = (ROOT/name).read_text(encoding='utf-8').replace('\r\n', '\n').encode()
            self.assertEqual(expected, hashlib.sha256(content).hexdigest(), name)

    def test_every_pinned_corpus_file_is_tracked_and_matches_lock(self):
        lock = json.loads((ROOT/'fixtures/omnia-lock.json').read_text())
        tracked = execute(['git', 'ls-files', '--', 'fixtures/omnia'])
        self.assertEqual(0, tracked.returncode, tracked.stderr)
        names = {Path(p).name for p in tracked.stdout.splitlines()}
        self.assertEqual(set(lock['files']), names)
        for name, expected in lock['files'].items():
            self.assertEqual(expected, hashlib.sha256((ROOT/'fixtures/omnia'/name).read_bytes()).hexdigest(), name)

    def test_cli_cannot_accept_mutation_commands_or_injected_action_fields(self):
        for command in ['TerminateFlow', 'route', 'proxy', 'tun', 'helper']:
            result = execute([str(BINARY), command])
            self.assertNotEqual(0, result.returncode)
            self.assertEqual('', result.stdout)
        manifest = json.loads((ROOT/'fixtures/omnia/manifest.json').read_text())
        for case in manifest['cases']:
            command = 'fixture' if case['kind'] == 'native' else 'observation'
            result = execute([str(BINARY), command, str(ROOT/'fixtures/omnia'/case['path']), manifest['now']])
            with self.subTest(case=case['id']):
                self.assertEqual(case['accepted'], result.returncode == 0, result.stderr)
                if case['accepted']:
                    output = json.loads(result.stdout)
                    self.assertEqual('NOT_EXECUTED', output['native_gate'])
                    self.assertEqual('observation_only', output['graph']['authority'])
                    for flow in output['graph']['flows']:
                        self.assertEqual('observation_only', flow['observation']['authority'])
                        self.assertNotEqual('native_local_query', flow['origin'])
                else:
                    self.assertEqual('', result.stdout)

    @unittest.skipUnless(os.name == 'nt', 'PE native import check NOT_EXECUTED on non-Windows host')
    def test_compiled_windows_imports_are_query_only(self):
        imports = pe_imports(BINARY)
        allowed_dlls = {'kernel32.dll', 'iphlpapi.dll', 'ntdll.dll', 'bcryptprimitives.dll',
            'api-ms-win-core-synch-l1-2-0.dll', 'vcruntime140.dll', 'vcruntime140_1.dll',
            'api-ms-win-crt-runtime-l1-1-0.dll', 'api-ms-win-crt-math-l1-1-0.dll',
            'api-ms-win-crt-stdio-l1-1-0.dll', 'api-ms-win-crt-locale-l1-1-0.dll',
            'api-ms-win-crt-heap-l1-1-0.dll'}
        self.assertLessEqual(set(imports), allowed_dlls)
        self.assertEqual(['GetExtendedTcpTable'], imports['iphlpapi.dll'])
        names = {name for functions in imports.values() for name in functions}
        self.assertTrue({'OpenProcess', 'GetProcessTimes', 'CloseHandle'} <= names)
        self.assertFalse(names & {'CreateProcessA', 'CreateProcessW', 'WinExec', 'ShellExecuteA',
            'ShellExecuteW', 'TerminateProcess', 'SetTcpEntry', 'SetIpForwardEntry',
            'CreateIpForwardEntry', 'CreateIpForwardEntry2', 'SetIpInterfaceEntry', 'DeleteIpForwardEntry'})
        # Rust/CRT retain file/console and dynamic-loader primitives. Their absence is
        # not claimed. Reviewed production source plus dependency closure bounds use.
        print('PE IP Helper imports:', ', '.join(imports['iphlpapi.dll']))


if __name__ == '__main__':
    unittest.main(verbosity=2)
