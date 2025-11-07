#!/usr/bin/env python3
"""
Port scanner script using socket programming.
"""
import socket
import sys
import threading


def scan_port(target, port):
    try:
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.settimeout(0.5) 
        result = s.connect_ex((target, port))
        if result == 0:
            print(f"Port {port}: OPEN")
        else:
            print(f"Port {port}: CLOSED")
    except Exception as e:
        print(f"Error checking port {port}: {e}")
    finally:
        s.close()


def main():
    target = sys.argv[1]
    start_port = int(sys.argv[2])
    end_port = int(sys.argv[3])

    print(f"Starting scan on {target} ports: {start_port}-{end_port}")
    threads = []
    for port in range(start_port, end_port + 1):
        t = threading.Thread(target=scan_port, args=(target, port))
        threads.append(t)
        t.start()

    for t in threads:
        t.join()


if __name__ == "__main__":
    if len(sys.argv) != 4:
        print("Usage: python3 port_scanner.py <target_ip> <start_port> <end_port>")
    else:
        main()
