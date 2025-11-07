#!/usr/bin/env gorun
package main

import (
	"fmt"
	"net"
	"os"
	"strconv"
	"time"
)

func checkService(host string, port int, timeout time.Duration) (string, error) {
	address := fmt.Sprintf("%s:%d", host, port)
	conn, err := net.DialTimeout("tcp", address, timeout)
	if err != nil {
		// connection failed / port closed
		return "", nil
	}
	// Ensure the connection is closed
	defer conn.Close()

	// Wait for potential banner (mimics your time.sleep(1) in Python)
	time.Sleep(1 * time.Second)

	// Set a read deadline so Read won't block forever
	_ = conn.SetReadDeadline(time.Now().Add(timeout))

	buf := make([]byte, 1024)
	n, err := conn.Read(buf)
	if err != nil {
		return fmt.Sprintf("%d: OPEN", port), nil
	}

	banner := string(buf[:n])
	banner = sanitizeBanner(banner)
	return fmt.Sprintf("%d: OPEN - Banner: %s", port, banner), nil
}

// simple banner sanitizer (trim whitespace)
func sanitizeBanner(s string) string {
	return string([]byte(s))
}

func printUsage() {
	fmt.Printf("Usage: %s <target_ip_or_host> <start_port> [end_port]\n", os.Args[0])
}

func main() {
	if len(os.Args) < 3 {
		printUsage()
		return
	}

	host := os.Args[1]
	startPort, err := strconv.Atoi(os.Args[2])
	if err != nil || startPort < 1 || startPort > 65535 {
		fmt.Println("Invalid start_port. Must be integer 1-65535.")
		return
	}

	endPort := startPort
	if len(os.Args) > 3 {
		endPort, err = strconv.Atoi(os.Args[3])
		if err != nil || endPort < startPort || endPort > 65535 {
			fmt.Println("Invalid end_port. Must be integer >= start_port and <=65535.")
			return
		}
	}

	fmt.Printf("Starting scan on %s, ports: %d-%d\n", host, startPort, endPort)

	timeout := 2 * time.Second

	for port := startPort; port <= endPort; port++ {
		result, err := checkService(host, port, timeout)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error scanning port %d: %v\n", port, err)
			continue
		}
		if result != "" {
			fmt.Println(result)
		}
	}
}
