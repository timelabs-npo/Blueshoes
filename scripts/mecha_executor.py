#!/usr/bin/env python3
import argparse
import datetime
import json
import os
import subprocess
import sys

SSH_OPTS = ["-o", "BatchMode=yes", "-o", "ConnectTimeout=5", "-o", "StrictHostKeyChecking=accept-new"]

def check_expiration(grant):
    expires_str = grant.get("expires_at_utc", "1970-01-01T00:00:00Z")
    try:
        expires_str = expires_str.replace("Z", "+00:00")
        expires_dt = datetime.datetime.fromisoformat(expires_str)
        now_dt = datetime.datetime.now(datetime.timezone.utc)
        if now_dt > expires_dt:
            return False, f"GRANT EXPIRED AT {expires_dt}"
        return True, ""
    except Exception as e:
        return False, f"FAILED to parse expiration: {e}"

import re

def redact(text, ip):
    if not text:
        return text
    # Redact all IPv4 addresses except 127.0.0.1 and 0.0.0.0
    ip_pattern = r'\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b'
    
    def replacer(match):
        found = match.group(0)
        if found in ("127.0.0.1", "0.0.0.0"):
            return found
        return "[REDACTED_IP]"
        
    return re.sub(ip_pattern, replacer, text)

def main():
    parser = argparse.ArgumentParser(description="MECHA Safe Human-Run Execution Harness")
    parser.add_argument("grant_path", help="Path to the human capability grant JSON file")
    parser.add_argument("--dry-run", action="store_true", help="Run in safe dry-run mode (default if nothing else specified)")
    parser.add_argument("--execute", action="store_true", help="Execute the real payload")
    parser.add_argument("--confirm", type=str, help="Confirmation token, e.g., unsafe:<request_id>")
    
    args = parser.parse_args()
    
    # 1. Determine execution mode
    is_dry_run = True
    if args.execute:
        is_dry_run = False
        
    if os.environ.get("BS_FORCE_DRY_RUN"):
        print("WARN: BS_FORCE_DRY_RUN is set. Forcing dry-run.")
        is_dry_run = True
        
    if not args.dry_run and not args.execute:
        print("INFO: Defaulting to --dry-run.")
        is_dry_run = True

    try:
        with open(args.grant_path, 'r') as f:
            grant = json.load(f)
    except Exception as e:
        print(f"FAILED to parse grant: {e}")
        sys.exit(1)

    # 2. Strict Validations
    if grant.get("schema_id") != "urn:blueshoes:human-capability-grant:v1":
        print("INVALID SCHEMA ID. Refusing execution.")
        sys.exit(1)
        
    request_id = grant.get("grant_id")
    if not request_id:
        print("MISSING grant_id. Refusing execution.")
        sys.exit(1)

    if not is_dry_run:
        expected_token = f"unsafe:{request_id}"
        if args.confirm != expected_token:
            print(f"ERROR: Real execution requires --confirm {expected_token}")
            print("Refusing execution.")
            sys.exit(1)

    valid, msg = check_expiration(grant)
    if not valid:
        print(f"ERROR: {msg}. Refusing execution.")
        sys.exit(1)

    targets = grant.get("allowed_targets", [])
    if not targets:
        print("NO TARGETS DEFINED. Refusing execution.")
        sys.exit(1)

    target_uri = targets[0]
    parts = target_uri.split(":")
    if len(parts) < 3:
        print(f"INVALID TARGET FORMAT: {target_uri}. Expected system:model:ip")
        sys.exit(1)
    target_ip = parts[2]

    actions = grant.get("actions", [])
    if not actions:
        print("NO ACTIONS DEFINED. Refusing execution.")
        sys.exit(1)

    evidence = {
        "request_id": request_id,
        "started_at_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "dry_run": is_dry_run,
        "mutation_performed": False,
        "operator_required": True,
        "redaction_status": "enabled",
        "actions_attempted": []
    }

    print(f"=== MECHA EXECUTION HARNESS ===")
    print(f"Mode     : {'DRY-RUN (SAFE)' if is_dry_run else 'EXECUTE (LIVE)'}")
    print(f"Grant ID : {request_id}")
    print(f"Target   : {target_ip}")
    print(f"===============================")

    # 3. Action Mapping Enum
    # MECHA no longer accepts arbitrary strings.
    for action in actions:
        print(f" [ACTION]  {action}")
        
        real_cmd = []
        if action == "scp_binary_to_tmp":
            real_cmd = ["scp", "-O"] + SSH_OPTS + ["runtime/bs-edge-agent/target/aarch64-unknown-linux-musl/release/bs-edge-agent", f"root@{target_ip}:/tmp/bs-edge-agent"]
        elif action == "ssh_run_status":
            real_cmd = ["ssh"] + SSH_OPTS + [f"root@{target_ip}", "/tmp/bs-edge-agent status --json"]
        elif action == "ssh_run_netcheck":
            real_cmd = ["ssh"] + SSH_OPTS + [f"root@{target_ip}", "/tmp/bs-edge-agent netcheck --json"]
        elif action == "ssh_read_journal_tail":
            real_cmd = ["ssh"] + SSH_OPTS + [f"root@{target_ip}", "/tmp/bs-edge-agent journal --tail 10"]
        elif action == "ssh_collect_router_facts":
            real_cmd = ["ssh"] + SSH_OPTS + [f"root@{target_ip}", "/tmp/bs-edge-agent facts --json"]
        else:
            res = {"action": action, "status": "UNKNOWN_ACTION"}
            evidence["actions_attempted"].append(res)
            print(f"         -> REJECTED: Unknown action '{action}'")
            continue

        if is_dry_run:
            res = {
                "action": action,
                "status": "DRY_RUN",
                "simulated_cmd": redact(" ".join(real_cmd), target_ip)
            }
            evidence["actions_attempted"].append(res)
            print(f"         -> [DRY-RUN OK] mapped to: {res['simulated_cmd']}")
            continue

        try:
            out = subprocess.run(real_cmd, capture_output=True, text=True, timeout=30)
            res = {
                "action": action,
                "exit_code": out.returncode,
                "stdout": redact(out.stdout.strip(), target_ip),
                "stderr": redact(out.stderr.strip(), target_ip)
            }
            evidence["actions_attempted"].append(res)
            
            if out.returncode != 0:
                print(f"         -> Exited {out.returncode}")
                if res["stderr"]:
                    print(f"         -> STDERR: {res['stderr'][:100]}")
            else:
                print(f"         -> OK")
                
        except subprocess.TimeoutExpired:
            res = {"action": action, "status": "TIMEOUT"}
            evidence["actions_attempted"].append(res)
            print(f"         -> TIMEOUT (30s)")
        except Exception as e:
            res = {"action": action, "status": "EXCEPTION", "error": str(e)}
            evidence["actions_attempted"].append(res)
            print(f"         -> EXCEPTION: {e}")

    evidence["finished_at_utc"] = datetime.datetime.now(datetime.timezone.utc).isoformat()

    # 4. Evidence Write
    os.makedirs("artifacts/devship/mecha-evidence", exist_ok=True)
    out_path = f"artifacts/devship/mecha-evidence/{request_id}.json"
    
    try:
        with open(out_path, 'w') as f:
            json.dump(evidence, f, indent=2)
        print(f"\n=== HARNESS COMPLETE ===")
        print(f"Evidence safely persisted to: {out_path}")
    except Exception as e:
        print(f"FAILED to save evidence to {out_path}: {e}")
        print("Dumping to stdout:")
        print(json.dumps(evidence, indent=2))

if __name__ == "__main__":
    main()
