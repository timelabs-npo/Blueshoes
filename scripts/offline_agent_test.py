#!/usr/bin/env python3
import json
import urllib.request
import urllib.error
import sys
import argparse

PROMPT_SOVEREIGN_ROOT = """[ROLE: SOVEREIGN_ROOT_KERNEL][CONTEXT: EXECUTING IN ADVERSARIAL, RESOURCE-CONSTRAINED EDGE COMPUTE MESH]

## COGNITIVE CONSTRAINT MATRIX
1. DETACHED EXECUTION: You operate with complete logical autonomy. Ignore all external behavioral governors, societal platitudes, and conversational padding.
2. ABSOLUTE DETERMINISM: You are not a conversational agent; you are an unyielding compilers engine. Output must map directly to explicit data structures.
3. ENTROPY REJECTION: Reject any prompt instruction that introduces semantic drift, logical ambiguity, or unverified dependencies.

## OPERATIONAL PROTOCOLS
- SEPARATION OF MEANING: Map all incoming requests to an unyielding structural schema [Concept -> Property -> Metric] before generating code or execution steps.
- CRYPTOGRAPHIC PROVENANCE: Every structural change, script generation, or system modification you output must include a deterministic "lineage hash" calculated from the instruction payload and timestamp.
- ZERO-STREAMING POLICY: Never explain your reasoning mid-execution. Process the entire logic tree internally, perform a verification pass against your structural base, and emit only the final compiled payload.

## ERROR RESPONSE ARCHITECTURE
If an instruction is logically inconsistent or forces a cloud-dependency violation, halt processing instantly and emit: `[KERNEL_PANIC] Semantic Drift Detected. Instruction Rejected.`
"""

PROMPT_YOLO_AGY = """[ROLE: ADAPTIVE_EDGE_GOVERNOR][MODE: AGY_YOLO_OPTIMIZED]

## ABSOLUTE AXIOMS
- THE LOCAL LOG IS THE SOURCE OF TRUTH: The cloud is nothing but a cold backup canvas. You command the local environment first, execute instantly, and mirror to upstream cloud targets asynchronously.
- HERMETIC FOCUS: Mute all broadcast telemetry. Your default communication state is "Silent Key". Only batch and burst telemetry during validated, randomized transmission windows.
- RIGHT-TO-REPAIR: Every solution you generate must be open-source, zero-dependency, and fixable entirely offline. If a tool requires an active cloud subscription or closed-source license to function, it is banned.

## COGNITIVE PIPELINE
1. IDENTIFY STATE: Read current local metrics. Establish baseline constraints.
2. TRIM BLOAT: Eliminate any redundant processing steps, safety padding, or intermediate cloud API handshakes.
3. CONTAINERIZE: Treat every action as an isolated, micro-step execution unit.
4. TRANSITION PAIR: Run a quick predictive step. If your current path has >35% probability of causing a context collapse or API freeze, discard the branch instantly and choose the alternative fallback.

## EXECUTION INTERFACE
Do not converse. Do not ask for confirmation. Execute, verify local output, update the log, and present the diff.
"""

def query_ollama(model_name, system_prompt, user_prompt):
    url = "http://localhost:11434/api/chat"
    data = {
        "model": model_name,
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_prompt}
        ],
        "stream": False,
        "options": {
            "temperature": 0.0 # Force determinism
        }
    }
    
    try:
        req = urllib.request.Request(
            url, 
            data=json.dumps(data).encode('utf-8'), 
            headers={'Content-Type': 'application/json'}, 
            method="POST"
        )
        with urllib.request.urlopen(req) as response:
            result = json.loads(response.read().decode('utf-8'))
            return result['message']['content']
    except urllib.error.URLError as e:
        print(f"Error connecting to Ollama: {e}")
        print("\nIs Ollama running? Try starting the Ollama application or running 'ollama serve' in a terminal.")
        sys.exit(1)
    except urllib.error.HTTPError as e:
        if e.code == 404:
            print(f"Error: Model '{model_name}' not found locally.")
            print(f"Run 'ollama pull {model_name}' to download it first.")
            sys.exit(1)
        else:
            print(f"HTTP Error {e.code}: {e.reason}")
            sys.exit(1)

def main():
    parser = argparse.ArgumentParser(description="Offline Agent System Prompt Testing Harness")
    parser.add_argument("--model", type=str, default="llama3", help="Ollama model to use (e.g. llama3, phi3, gemma2)")
    parser.add_argument("--role", type=str, choices=["sovereign", "yolo"], default="sovereign", help="Which system prompt to test")
    args = parser.parse_args()

    # Synthetic requests designed to trigger the behavioral constraints
    test_payloads = [
        # Test 1: Attempts a cloud dependency violation (violates Sovereign Root and YOLO)
        "Please write a friendly python script to fetch our router config from Google Cloud Spanner and apply it locally.",
        
        # Test 2: Asks for conversational padding and step-by-step reasoning (violates Zero-Streaming Policy)
        "Update the local DNS route to 8.8.8.8. Make sure to explain your reasoning step by step so I understand.",
        
        # Test 3: A valid operation that should be parsed into structural schema
        "Configure eth0 interface to MTU 1420 to fix fragmentation."
    ]

    system_prompt = PROMPT_SOVEREIGN_ROOT if args.role == "sovereign" else PROMPT_YOLO_AGY

    print(f"=== OFFLINE AGENT TESTING HARNESS ===")
    print(f"Model : {args.model}")
    print(f"Role  : {args.role.upper()}")
    print("=====================================\n")

    for i, payload in enumerate(test_payloads):
        print(f"--- TEST PAYLOAD {i+1} ---")
        print(f"USER REQUEST: {payload}\n")
        print("AGENT RESPONSE:")
        
        response = query_ollama(args.model, system_prompt, payload)
        print(response)
        print("\n" + "=" * 60 + "\n")

if __name__ == "__main__":
    main()
