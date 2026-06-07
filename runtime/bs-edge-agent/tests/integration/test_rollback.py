import os
import pexpect
import pytest
import time
import urllib.request
import subprocess

OPENWRT_VERSION = "23.05.3"
OPENWRT_ARCH = "x86_64"
IMG_URL = f"https://downloads.openwrt.org/releases/{OPENWRT_VERSION}/targets/x86/64/openwrt-{OPENWRT_VERSION}-x86-64-generic-ext4-combined.img.gz"
IMG_GZ_NAME = "openwrt.img.gz"
IMG_NAME = "openwrt.img"

def download_openwrt():
    if not os.path.exists(IMG_NAME):
        print("Downloading OpenWrt image...")
        urllib.request.urlretrieve(IMG_URL, IMG_GZ_NAME)
        subprocess.run(["gunzip", IMG_GZ_NAME], check=True)
        print("Download complete.")

@pytest.fixture(scope="module")
def qemu_openwrt():
    download_openwrt()
    
    # Boot QEMU with the OpenWrt image and redirect serial to stdio
    qemu_cmd = f"qemu-system-x86_64 -m 256 -drive file={IMG_NAME},format=raw -nographic -nic user,hostfwd=tcp::2222-:22"
    child = pexpect.spawn(qemu_cmd, encoding='utf-8', timeout=60)
    
    # Wait for the boot to finish and press Enter to activate console
    child.expect("Please press Enter to activate this console.", timeout=60)
    child.sendline()
    child.expect("root@OpenWrt:/#")
    
    yield child
    
    # Teardown
    child.sendline("poweroff")
    child.close()

def test_watchdog_rollback(qemu_openwrt):
    child = qemu_openwrt
    
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
    child.expect("root@OpenWrt:/#")
    
    # Setup initial state
    child.sendline("cp /etc/config/network /etc/config.backup")
    child.expect("root@OpenWrt:/#")
    
    # Fire the watchdog in the background
    child.sendline("/bin/bs-watchdog &")
    child.expect("root@OpenWrt:/#")
    
    # Perform dangerous mutation (blackhole routing)
    child.sendline("ip route add blackhole 0.0.0.0/0")
    child.expect("root@OpenWrt:/#")
    
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

def test_masque_tunnel(qemu_openwrt):
    child = qemu_openwrt
    
    # Simulate execution of the new CapabilityGraph that spins up a MASQUE tunnel
    child.sendline("echo 'Simulating MASQUE proxy daemon...' > /var/log/masque.log")
    child.sendline("ip link add masque0 type dummy")
    child.sendline("ip link set masque0 up")
    child.expect("root@OpenWrt:/#")
    
    # Add a route forcing traffic through the MASQUE tunnel
    child.sendline("ip route add 8.8.8.8 dev masque0")
    child.expect("root@OpenWrt:/#")
    
    # Verify the route was created successfully
    child.sendline("ip route | grep masque0")
    child.expect("8.8.8.8 dev masque0")
    
    # Clean up
    child.sendline("ip route del 8.8.8.8 dev masque0")
    child.sendline("ip link del masque0")
    child.expect("root@OpenWrt:/#")
