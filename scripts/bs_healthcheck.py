#!/usr/bin/env python3
import os
import json
import re
import datetime
import subprocess
from pathlib import Path

# Paths
ROOT_DIR = Path(__file__).parent.parent.resolve()
DOCS_DIR = ROOT_DIR / "docs"
SRC_DIR = ROOT_DIR / "runtime/bs-edge-agent/src"
ARTIFACTS_DIR = ROOT_DIR / "artifacts"

def get_git_info():
    try:
        commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT_DIR, text=True).strip()
        branch = subprocess.check_output(["git", "rev-parse", "--abbrev-ref", "HEAD"], cwd=ROOT_DIR, text=True).strip()
        return commit, branch
    except Exception:
        return "unknown", "unknown"

def check_unreferenced_docs():
    findings = []
    if not DOCS_DIR.exists():
        return findings

    all_docs = {f.name for f in DOCS_DIR.glob("**/*.md") if f.name != "index.md"}
    readme_path = ROOT_DIR / "README.md"
    index_path = DOCS_DIR / "index.md"
    
    referenced = set()
    for file_to_check in [readme_path, index_path]:
        if file_to_check.exists():
            content = file_to_check.read_text()
            for doc in all_docs:
                if doc in content:
                    referenced.add(doc)

    unreferenced = all_docs - referenced
    for doc in unreferenced:
        findings.append({
            "id": f"HLT-DOC-{hash(doc) % 100000:05d}",
            "category": "dead_leftover",
            "severity": "medium",
            "title": f"Unreferenced documentation file: {doc}",
            "evidence": [f"docs/{doc}"],
            "impact": "Documentation drift and clutter",
            "remediation": {
                "steps": ["Link this doc in README.md or docs/index.md, or delete it."],
                "owner_hint": "docs",
                "issue_template": "tech-debt"
            }
        })
    return findings

def check_todo_without_issue():
    findings = []
    todo_re = re.compile(r'(TODO|FIXME)(?!.*#\d+)')
    
    for path in ROOT_DIR.rglob("*.rs"):
        if "target" in path.parts:
            continue
        try:
            lines = path.read_text(encoding='utf-8', errors='ignore').splitlines()
            for i, line in enumerate(lines):
                if todo_re.search(line):
                    findings.append({
                        "id": f"HLT-TODO-{hash(str(path)+str(i)) % 100000:05d}",
                        "category": "goal_alignment",
                        "severity": "low",
                        "title": "TODO/FIXME without issue number",
                        "evidence": [f"{path.relative_to(ROOT_DIR)}#L{i+1}"],
                        "impact": "Untracked technical debt",
                        "remediation": {
                            "steps": ["Create a GitHub issue and append its number (e.g., TODO #123)"],
                            "owner_hint": "runtime",
                            "issue_template": "tech-debt"
                        }
                    })
        except Exception:
            pass
    return findings

def check_self_violations():
    findings = []
    mutating_patterns = [
        "uci set", "uci commit", "nft add", "nft delete", 
        "iptables", "ip route add", "ip route del", "wg set"
    ]
    
    for path in ROOT_DIR.rglob("*.rs"):
        if "target" in path.parts:
            continue
            
        rel_path = path.relative_to(ROOT_DIR)
        
        # Executor is the only place allowed to use mutations
        if "executor/freebsd.rs" in str(rel_path).lower():
            continue
            
        # Ignore the audit test itself which contains the string patterns
        if "tests/hygiene_linter_test.rs" in str(rel_path) or "tests/audit_test.rs" in str(rel_path):
            continue
            
        try:
            lines = path.read_text(encoding='utf-8', errors='ignore').splitlines()
            for i, line in enumerate(lines):
                if line.strip().startswith("//"):
                    continue # skip comments
                for pattern in mutating_patterns:
                    if pattern in line:
                        findings.append({
                            "id": f"HLT-SEC-{hash(str(path)+pattern) % 100000:05d}",
                            "category": "self_violation",
                            "severity": "critical",
                            "title": f"Illegal mutation command '{pattern}' outside executor boundary",
                            "evidence": [f"{rel_path}#L{i+1}", line.strip()],
                            "impact": "Bypasses zero-mutation safety constraints and risks bricking router",
                            "remediation": {
                                "steps": ["Remove the mutation or move it into the isolated executor boundary."],
                                "owner_hint": "runtime",
                                "issue_template": "security"
                            }
                        })
        except Exception:
            pass
    return findings

def main():
    commit, branch = get_git_info()
    
    findings = []
    findings.extend(check_unreferenced_docs())
    findings.extend(check_todo_without_issue())
    findings.extend(check_self_violations())

    summary = {
        "critical": sum(1 for f in findings if f["severity"] == "critical"),
        "high": sum(1 for f in findings if f["severity"] == "high"),
        "medium": sum(1 for f in findings if f["severity"] == "medium"),
        "low": sum(1 for f in findings if f["severity"] == "low"),
    }

    report = {
        "schema_id": "urn:bs:healthcheck:v1",
        "run": {
            "timestamp_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
            "git_sha": commit,
            "branch": branch,
            "runner": os.environ.get("GITHUB_ACTIONS", "local") == "true" and "github-actions" or "local"
        },
        "summary": summary,
        "findings": findings,
        "trend_keys": {
            "dead_leftover_count": sum(1 for f in findings if f["category"] == "dead_leftover"),
            "rollback_violation_count": sum(1 for f in findings if f["category"] == "self_violation"),
            "doc_staleness_count": 0
        }
    }

    ARTIFACTS_DIR.mkdir(exist_ok=True)
    report_path = ARTIFACTS_DIR / "bs_health_report.json"
    report_path.write_text(json.dumps(report, indent=2))

    print(f"Healthcheck complete. Found {len(findings)} issues.")
    print(f"Critical: {summary['critical']}, High: {summary['high']}, Medium: {summary['medium']}, Low: {summary['low']}")
    print(f"Report saved to {report_path}")

    if summary["critical"] > 0:
        print("\n[!] CRITICAL VIOLATIONS FOUND. FAILING CI.")
        exit(1)

if __name__ == "__main__":
    main()
