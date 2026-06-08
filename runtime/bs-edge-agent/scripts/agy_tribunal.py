import asyncio
import json
import os
import sys
from pathlib import Path
from google.antigravity import Agent, LocalAgentConfig, types

async def run_tribunal(request_file_path: str, output_file_path: str):
    # Ensure the input file exists
    if not os.path.exists(request_file_path):
        print(f"Error: Pending request file not found at {request_file_path}")
        sys.exit(1)
        
    with open(request_file_path, "r") as f:
        request_data = f.read()

    print("Initializing AGY Orchestrator with Subagents enabled...")
    
    # Configure the main agent to allow subagents
    config = LocalAgentConfig(
        capabilities=types.CapabilitiesConfig(
            enable_subagents=True,
        )
    )

    system_prompt = """
You are the Rhea Tribunal Orchestrator. 
Your job is to read a structured JSON review request for a proposed code transaction and orchestrate a multi-agent tribunal to evaluate it.
You MUST follow the "Decision Leak Prevention" doctrine: Subagents must evaluate the request independently and must not see each other's verdicts.

1. Spawn THREE separate subagents:
   - Security Reviewer
   - Governance Reviewer
   - Architecture Consistency Reviewer
2. Feed each subagent the JSON request. Ask them to evaluate the request and return their findings in the `review_verdict.json` format.
3. Collect their responses.
4. Synthesize the final result into a `tribunal-verdict.md` format (Markdown).
    - Status can be PASS, REJECT, or WARN.
    - Provide reasoning based on the subagent findings.
    - Highlight any Bounded Profile Engine or Zero Runtime Mutation doctrine violations.
    - Provide Remediation Action Items if necessary.

Output exactly the Markdown content for the final verdict and nothing else.
"""

    async with Agent(config=config, system_instruction=system_prompt) as agent:
        print("Dispatching request to local model team...")
        prompt = f"Please conduct a tribunal review on this request:\n```json\n{request_data}\n```"
        
        response = await agent.chat(prompt)
        verdict_content = await response.text()
        
        # Write the final verdict out
        with open(output_file_path, "w") as f:
            f.write(verdict_content)
            
        print(f"Tribunal verdict saved to {output_file_path}")

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python agy_tribunal.py <path_to_request_json> <path_to_output_md>")
        sys.exit(1)
        
    request_file = sys.argv[1]
    output_file = sys.argv[2]
    
    asyncio.run(run_tribunal(request_file, output_file))
