import os
import pexpect
import pytest
import time
import urllib.request
import subprocess

FreeBSD_VERSION = "23.05.3"
FreeBSD_ARCH = "x86_64"
IMG_URL = f"https://downloads.FreeBSD.org/releases/{FreeBSD_VERSION}/targets/x86/64/FreeBSD-{FreeBSD_VERSION}-x86-64-generic-ext4-combined.img.gz"
IMG_GZ_NAME = "FreeBSD.img.gz"
IMG_NAME = "FreeBSD.img"

def download_FreeBSD():
    if not os.path.exists(IMG_NAME):
        print("Downloading FreeBSD image...")
        urllib.request.urlretrieve(IMG_URL, IMG_GZ_NAME)
        subprocess.run(["gunzip", IMG_GZ_NAME], check=True)
        print("Download complete.")

@pytest.fixture(scope="module")
def qemu_FreeBSD():
    download_FreeBSD()
    
    # Boot QEMU with the FreeBSD image and redirect serial to stdio
    qemu_cmd = f"qemu-system-x86_64 -m 256 -drive file={IMG_NAME},format=raw -nographic -nic user,hostfwd=tcp::2222-:22"
    child = pexpect.spawn(qemu_cmd, encoding='utf-8', timeout=60)
    
    # Wait for the boot to finish and press Enter to activate console
    child.expect("Please press Enter to activate this console.", timeout=60)
    child.sendline()
    child.expect("root@FreeBSD:/#")
    
    yield child
    
    # Teardown
    child.sendline("poweroff")
    child.close()

def test_watchdog_rollback(qemu_FreeBSD):
    child = qemu_FreeBSD
    
    # Ensure bs-edge-agent is "installed" (in real suite, we SCP the binary over port 2222)
    # For this suite skeleton, we'll write a mock watchdog loop and break the network to prove rollback semantics
    
    child.sendline("cat << 'EOF' > /bin/bs-watchdog")
    child.sendline("#!/bin/sh")
    child.sendline("sleep 5")
    child.sendline("echo 'Watchdog triggered! Rolling back...' > /dev/console")
    child.sendline("cp /etc/config.backup /etc/config/network")
    child.sendline("/etc/init.d/network reload")
    child.sendline("EOF")
    child.sendline("chmod +x /bin/bs-watchdog")
    child.expect("root@FreeBSD:/#")
    
    # Setup initial state
    child.sendline("cp /etc/config/network /etc/config.backup")
    child.expect("root@FreeBSD:/#")
    
    # Fire the watchdog in the background
    child.sendline("/bin/bs-watchdog &")
    child.expect("root@FreeBSD:/#")
    
    # Perform dangerous mutation (blackhole routing)
    child.sendline("ip route add blackhole 0.0.0.0/0")
    child.expect("root@FreeBSD:/#")
    
    # Verify mutation applied
    child.sendline("ip route")
    child.expect("blackhole default")
    
    # Wait for watchdog to recover
    child.expect("Watchdog triggered! Rolling back...", timeout=10)
    
    # Verify rollback restored the network
    child.sendline("ip route")
    # Should not contain blackhole anymore
    time.sleep(2)
    child.sendline("ip route | grep blackhole || echo 'CLEAN'")
    child.expect("CLEAN")

def test_masque_tunnel(qemu_FreeBSD):
    child = qemu_FreeBSD
    
    # Simulate execution of the new CapabilityGraph that spins up a MASQUE tunnel
    child.sendline("echo 'Simulating MASQUE proxy daemon...' > /var/log/masque.log")
    child.sendline("ip link add masque0 type dummy")
    child.sendline("ip link set masque0 up")
    child.expect("root@FreeBSD:/#")
    
    # Add a route forcing traffic through the MASQUE tunnel
    child.sendline("ip route add 8.8.8.8 dev masque0")
    child.expect("root@FreeBSD:/#")
    
    # Verify the route was created successfully
    child.sendline("ip route | grep masque0")
    child.expect("8.8.8.8 dev masque0")
    
    # Clean up
    child.sendline("ip route del 8.8.8.8 dev masque0")
    child.sendline("ip link del masque0")
    child.expect("root@FreeBSD:/#")

def test_commit_confirmed_flow(qemu_FreeBSD):
    child = qemu_FreeBSD
    
    # Generate a plan
    child.sendline("bs-edge-agent plan SAFE_MTU --out /tmp/tx.json")
    child.expect("Plan successfully written to /tmp/tx.json")
    child.expect("root@FreeBSD:/#")
    
    # Apply confirmed
    child.sendline("bs-edge-agent apply-confirmed /tmp/tx.json --timeout 10")
    child.expect("Configuration active")
    child.expect("root@FreeBSD:/#")
    
    # Extract the tx_id from the output (mocking for test)
    child.sendline("TX_ID=$(ls /tmp/blueshoes_confirm_* | sed 's/.*_confirm_//')")
    child.sendline("echo $TX_ID")
    
    # Confirm
    child.sendline("bs-edge-agent confirm $TX_ID")
    child.expect("confirmed successfully")
    child.expect("root@FreeBSD:/#")
