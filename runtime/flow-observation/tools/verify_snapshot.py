"""Validate actual native output against pinned data schema; print aggregates only."""
import json
import os
from pathlib import Path
import subprocess

from jsonschema import Draft202012Validator, FormatChecker

root = Path(__file__).resolve().parents[1]
if os.name != 'nt':
    raise SystemExit('Windows native snapshot NOT_EXECUTED')
if 'date-time' not in FormatChecker().checkers:
    raise SystemExit('date-time validator missing')
binary = root/'target/release/bs-flow-observation.exe'
result = subprocess.run([str(binary), 'snapshot'], capture_output=True, text=True, timeout=30)
if result.returncode:
    raise SystemExit('native collector failed; no PASS receipt')
snapshot = json.loads(result.stdout)
validator = Draft202012Validator(json.loads((root/'fixtures/omnia/flow-observation.schema.json').read_text()), format_checker=FormatChecker())
batch, graph = snapshot['batch'], snapshot['graph']
assert batch['native_gate'] == 'PASS'
assert graph['authority'] == 'observation_only'
assert len(batch['observations']) == len(graph['flows'])
for item in batch['observations']:
    assert item['origin'] == 'native_local_query'
    assert validator.is_valid(item['observation']), 'native wire schema violation'
    assert item['observation']['counters']['bytes_up'] is None
    assert item['observation']['counters']['bytes_down'] is None
for flow in graph['flows']:
    assert validator.is_valid(flow['observation']), 'graph wire schema violation'
    assert flow['origin'] == 'native_local_query'
    assert flow['relation'] == 'endpoint_association'
    assert flow['traffic_direction'] == 'unknown'
bound = sum(item['observation']['process_ref'] is not None for item in batch['observations'])
print(json.dumps({'native_wire_schema':'PASS', 'graph_wire_schema':'PASS',
    'tcp_rows':len(batch['observations']), 'process_bound':bound,
    'process_unknown':len(batch['observations'])-bound,
    'native_gate':'PASS', 'schema_revision':'2026-09-06-null-counters',
    'authority':'observation_only', 'collection_interval':batch['collection_interval'],
    'coverage_note':'Empty samples do not establish live process-association coverage' if not bound else 'Live process association observed; fixture checks establish boundary cases'}))
