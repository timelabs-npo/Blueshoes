#!/usr/bin/env python3
import os
import sys
import glob
import json
import urllib.request
import urllib.error

# Run this with: OPENROUTER_API_KEY="sk-or-v1-..." python3 scripts/multi_model_audit.py

API_KEY = os.environ.get("OPENROUTER_API_KEY")
if not API_KEY:
    print("ERROR: OPENROUTER_API_KEY environment variable is not set.")
    sys.exit(1)

# A blend of the cheapest high-quality models
MODELS = [
    "anthropic/claude-3.5-sonnet",
    "google/gemini-pro-1.5",
    "openai/gpt-4o",
    "meta-llama/llama-3-70b-instruct"
]

SYSTEM_PROMPT = """You are a senior security researcher specializing in embedded systems, Linux networking, and Rust.
You are auditing a daemon that runs as root on an FreeBSD home router (GL-MT3000).
Your job is to identify logic flaws, path traversal vectors, unchecked assumptions, and command injection vulnerabilities.
The agent uses 'uci batch' via stdin to mutate state, and has a strict 'no shell interpolation' governance policy.
Please output a concise markdown report of your findings categorized by severity."""

def collect_source_code():
    code_bundle = ""
    # Look for rust files in the bs-edge-agent runtime
    pattern = os.path.join("runtime", "bs-edge-agent", "src", "**", "*.rs")
    files = glob.glob(pattern, recursive=True)
    
    if not files:
        print("WARN: No .rs files found in runtime/bs-edge-agent/src/")
        
    for filepath in files:
        with open(filepath, 'r') as f:
            content = f.read()
            code_bundle += f"\n\n--- FILE: {filepath} ---\n```rust\n{content}\n```\n"
            
    # Also grab the watchdog
    watchdog_path = os.path.join("runtime", "bs-edge-agent", "src", "watchdog.rs")
    if os.path.exists(watchdog_path) and watchdog_path not in files:
        with open(watchdog_path, 'r') as f:
            code_bundle += f"\n\n--- FILE: {watchdog_path} ---\n```rust\n{f.read()}\n```\n"

    return code_bundle

def query_openrouter(model, system_prompt, user_prompt):
    url = "https://openrouter.ai/api/v1/chat/completions"
    headers = {
        "Authorization": f"Bearer {API_KEY}",
        "HTTP-Referer": "https://github.com/timelabs-npo/Blueshoes",
        "X-Title": "Blueshoes Multi-Model Audit",
        "Content-Type": "application/json"
    }
    
    data = {
        "model": model,
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_prompt}
        ]
    }
    
    req = urllib.request.Request(url, data=json.dumps(data).encode('utf-8'), headers=headers, method="POST")
    try:
        with urllib.request.urlopen(req) as response:
            result = json.loads(response.read().decode('utf-8'))
            return result['choices'][0]['message']['content']
    except urllib.error.HTTPError as e:
        return f"HTTP Error: {e.code} - {e.read().decode('utf-8')}"
    except Exception as e:
        return f"Error: {str(e)}"

def main():
    print("Collecting source code...")
    code_bundle = collect_source_code()
    if not code_bundle.strip():
        print("No code to audit. Make sure you're running this from the repository root.")
        sys.exit(1)
        
    user_prompt = f"Here is the complete source code of the agent. Please audit it:\n\n{code_bundle}"
    
    print(f"Loaded {len(code_bundle)} bytes of source code.")
    print(f"Starting audit across {len(MODELS)} models. This will take a few minutes...\n")
    
    os.makedirs("artifacts/audits", exist_ok=True)
    
    for model in MODELS:
        safe_name = model.replace("/", "_")
        print(f"-> Submitting to {model}...")
        
        response = query_openrouter(model, SYSTEM_PROMPT, user_prompt)
        
        out_path = f"artifacts/audits/{safe_name}_audit.md"
        with open(out_path, 'w') as f:
            f.write(f"# Audit by {model}\n\n")
            f.write(response)
            
        print(f"   Saved to {out_path}")
        
    print("\n✅ Multi-model audit complete. Compare the outputs in the artifacts/audits/ directory.")

if __name__ == "__main__":
    main()
