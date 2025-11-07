# 🔍 Web & Network Vulnerability Testing Toolkit

A collection of open-source security testing scripts designed for educational and authorized penetration testing purposes only.  
Each tool focuses on a specific class of web or network vulnerability, implemented in multiple programming languages for demonstration and learning.

> ⚠️ **Disclaimer:** These scripts are intended for **ethical use only**. Do not test systems without **explicit written permission** from the owner. Unauthorized testing is illegal and punishable under cybersecurity laws.

---

## 📁 Project Structure

| File | Language | Description |
|------|-----------|-------------|
| `web_vuln_test.go` | Go | Tests for **SQL Injection** vulnerabilities by injecting common payloads and measuring response delays. |
| `authentication_bypass.cs` | C# | Attempts **authentication bypass** using SQL-based payloads and token manipulation. |
| `AuthTest.java` | Java | Placeholder for **authentication logic and vulnerability testing** (supports integration with other modules). |
| `brute-force-login-attack.py` | Python | Performs **brute-force login testing** against a web form using user/password lists. |
| `cmd_injection.js` | Node.js | Checks for **command injection** vulnerabilities via crafted input parameters. |
| `error_handling_check.py` | Python | Verifies **error disclosure** issues and weak exception handling in web responses. |
| `port-scanner.py` | Python | Implements a **multi-threaded TCP port scanner** to identify open ports. |
| `vulnerable_services.go` | Go | Detects **open services** and extracts banners from network ports. |
| `weak-SSL_TSL-cipher-detection.rs` | Rust | (Planned/Experimental) Detects **weak SSL/TLS cipher suites** in target systems. |

---

## 🧰 Features

- 🧩 Multi-language examples: Go, Python, C#, Java, Node.js, Rust  
- 🧪 Focused vulnerability checks: SQLi, Auth Bypass, Command Injection, etc.  
- ⚙️ Easy-to-run CLI utilities with clear usage instructions  
- 🧵 Threaded or async implementations for efficient scanning  
- 🛡️ Built-in error handling and timeout management  

---

## 🚀 Quick Start

### Clone the repository
```bash
git clone https://github.com/<your-username>/<your-repo-name>.git
cd <your-repo-name>
```

### Run a specific tool

#### 🧠 SQL Injection (Go)
```bash
go run web_vuln_test.go https://target-site.com/page?id=
```

#### 🔑 Auth Bypass (C#)
```bash
dotnet run --project authentication_bypass.csproj https://target-site.com/login
```

#### 💥 Brute-force Login (Python)
```bash
python3 brute-force-login-attack.py https://target-site.com/login users.txt passwords.txt
```

#### ⚙️ Command Injection (Node.js)
```bash
node cmd_injection.js "https://target-site.com/test"
```

#### 🧱 Port Scanner (Python)
```bash
python3 port-scanner.py 192.168.0.1 20 1024
```

#### 🌐 Vulnerable Services (Go)
```bash
go run vulnerable_services.go 192.168.0.1 21 100
```

---

## 📚 Requirements

- **Go** ≥ 1.18  
- **Python** ≥ 3.8  
- **.NET SDK** ≥ 6.0  
- **Node.js** ≥ 18  
- **Rust (optional)** ≥ 1.70  

---

## 🧑‍💻 Contributing

Pull requests are welcome for:
- New vulnerability modules  
- Efficiency or concurrency improvements  
- Language-specific rewrites  

Ensure code adheres to:
- Ethical testing principles  
- Clean code standards  
- Non-destructive payloads  

---

## ⚖️ License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.
