#!/usr/bin/env python3
import os
import sys
import json
import hashlib
import urllib.request
import ssl
from datetime import datetime, timezone

# Paths
ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
RFC_DIR = os.path.join(ROOT_DIR, "rfc-library")
CATALOG_PATH = os.path.join(RFC_DIR, "catalog.json")

# Default bootstrap RFCs for the Blueshoes Gems Factory
DEFAULT_RFCS = {
    9000: "QUIC: A UDP-Based Multiplexed and Secure Transport",
    9298: "Proxying IP in HTTP/3 (MASQUE)",
    8446: "The Transport Layer Security (TLS) Protocol Version 1.3",
    9230: "Oblivious DNS over HTTPS (ODoH)",
    1035: "Domain Names - Implementation and Specification"
}

def get_sha256(filepath):
    hasher = hashlib.sha256()
    with open(filepath, "rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            hasher.update(chunk)
    return hasher.hexdigest()

def download_rfc(rfc_number):
    url = f"https://www.ietf.org/rfc/rfc{rfc_number}.txt"
    dest = os.path.join(RFC_DIR, f"rfc{rfc_number}.txt")
    
    # If already cached locally, skip download
    if os.path.exists(dest) and os.path.getsize(dest) > 0:
        print(f"RFC {rfc_number} already exists locally at {dest}. Skipping download.")
        return dest
        
    print(f"Downloading RFC {rfc_number} from {url}...")
    try:
        # Create unverified SSL context to bypass verification failures
        context = ssl._create_unverified_context()
        # User-agent header to avoid blocking by some network filters
        req = urllib.request.Request(
            url, 
            headers={'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64)'}
        )
        with urllib.request.urlopen(req, timeout=15, context=context) as response:
            content = response.read()
            with open(dest, "wb") as f:
                f.write(content)
        print(f"Successfully downloaded RFC {rfc_number} to {dest}.")
        return dest
    except Exception as e:
        print(f"Error downloading RFC {rfc_number}: {e}", file=sys.stderr)
        return None

def main():
    # Ensure catalog/RFC directory exists
    os.makedirs(RFC_DIR, exist_ok=True)
    
    # Parse arguments
    args = sys.argv[1:]
    target_rfcs = []
    
    if not args or "--default" in args:
        target_rfcs = list(DEFAULT_RFCS.keys())
    else:
        for arg in args:
            try:
                target_rfcs.append(int(arg))
            except ValueError:
                print(f"Warning: Ignoring non-numeric argument '{arg}'", file=sys.stderr)
                
    if not target_rfcs:
        print("No valid RFC numbers specified. Exiting.")
        sys.exit(1)
        
    # Load existing catalog
    catalog = {}
    if os.path.exists(CATALOG_PATH):
        try:
            with open(CATALOG_PATH, "r", encoding="utf-8") as f:
                entries = json.load(f)
                # If catalog is a list, convert to dict keyed by rfc_number for easy updates
                if isinstance(entries, list):
                    for entry in entries:
                        catalog[int(entry["rfc_number"])] = entry
                elif isinstance(entries, dict):
                    # In case it was stored as dict
                    for k, v in entries.items():
                        catalog[int(k)] = v
        except Exception as e:
            print(f"Warning: Failed to load catalog.json: {e}. Starting fresh.", file=sys.stderr)

    for rfc_num in target_rfcs:
        filepath = download_rfc(rfc_num)
        if filepath and os.path.exists(filepath):
            sha256_hash = get_sha256(filepath)
            title = DEFAULT_RFCS.get(rfc_num, f"RFC {rfc_num}")
            
            catalog[rfc_num] = {
                "rfc_number": rfc_num,
                "title": title,
                "sha256": sha256_hash,
                "path": f"rfc-library/rfc{rfc_num}.txt",
                "ingested_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
            }
        else:
            print(f"Failed to ingest RFC {rfc_num}.", file=sys.stderr)
            
    # Serialize catalog sorted by RFC number to guarantee deterministic output
    sorted_catalog = [catalog[k] for k in sorted(catalog.keys())]
    
    try:
        with open(CATALOG_PATH, "w", encoding="utf-8") as f:
            json.dump(sorted_catalog, f, indent=2)
        print(f"Catalog updated successfully at {CATALOG_PATH}")
    except Exception as e:
        print(f"Error writing catalog: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
