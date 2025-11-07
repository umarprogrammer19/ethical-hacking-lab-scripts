use openssl::ssl::{SslConnector, SslMethod, SslVersion};
use std::env;
use std::io::{self};
use std::net::{SocketAddr, ToSocketAddrs, TcpStream};
use std::time::Duration;

fn try_connect_with_cipher(host: &str, port: u16, item: &str, timeout: Duration) -> Result<Option<String>, String> {

    let mut builder = SslConnector::builder(SslMethod::tls()).map_err(|e| format!("builder error: {}", e))?;

    if item.to_lowercase().starts_with("tlsv1") {
        let version_token = item.to_lowercase().replace("tlsv", "");
        match version_token.as_str() {
            "1" | "1.0" => {
                builder.set_min_proto_version(Some(SslVersion::TLS1)).map_err(|e| format!("set min proto err: {}", e))?;
                builder.set_max_proto_version(Some(SslVersion::TLS1)).map_err(|e| format!("set max proto err: {}", e))?;
            }
            "1.1" => {
                builder.set_min_proto_version(Some(SslVersion::TLS1_1)).map_err(|e| format!("set min proto err: {}", e))?;
                builder.set_max_proto_version(Some(SslVersion::TLS1_1)).map_err(|e| format!("set max proto err: {}", e))?;
            }
            "1.2" => {
                builder.set_min_proto_version(Some(SslVersion::TLS1_2)).map_err(|e| format!("set min proto err: {}", e))?;
                builder.set_max_proto_version(Some(SslVersion::TLS1_2)).map_err(|e| format!("set max proto err: {}", e))?;
            }
            "1.3" => {
                builder.set_min_proto_version(Some(SslVersion::TLS1_3)).map_err(|e| format!("set min proto err: {}", e))?;
                builder.set_max_proto_version(Some(SslVersion::TLS1_3)).map_err(|e| format!("set max proto err: {}", e))?;
            }
            _ => {
                return Err(format!("Unrecognized TLS version token: {}", item));
            }
        }
    } else {
        builder
            .set_cipher_list(item)
            .map_err(|e| format!("failed to set cipher list '{}': {}", item, e))?;
    }

    let connector = builder.build();


    let addr_iter = (host, port).to_socket_addrs().map_err(|e| format!("resolve error: {}", e))?;

    let addr = addr_iter
        .into_iter()
        .next()
        .ok_or_else(|| "no address found".to_string())?;


    let stream = TcpStream::connect_timeout(&addr, timeout).map_err(|e| format!("tcp connect failed: {}", e))?;
    stream
        .set_read_timeout(Some(timeout))
        .map_err(|e| format!("set_read_timeout failed: {}", e))?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(|e| format!("set_write_timeout failed: {}", e))?;


    match connector.connect(host, stream) {
        Ok(ssl_stream) => {

            let ssl = ssl_stream.ssl();
            let cipher = ssl
                .current_cipher()
                .and_then(|c| c.name().map(|s| s.to_string()))
                .unwrap_or_else(|| "<unknown-cipher>".to_string());
            let version = ssl
                .version_str()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "<unknown-version>".to_string());
            Ok(Some(format!("{} => {} / {}", item, cipher, version)))
        }
        Err(err) => {

            Err(format!("handshake failed for '{}': {}", item, err))
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        eprintln!("Usage: {} <host> [port]", args[0]);
        eprintln!("Example: {} example.com 443", args[0]);
        std::process::exit(1);
    }

    let host = &args[1];
    let port: u16 = if args.len() >= 3 {
        args[2].parse().unwrap_or(443)
    } else {
        443
    };

    // Items to check: cipher names or protocol targets (TLSv1.0 etc.)
    let items = vec![
        "RC4-SHA",
        "DES-CBC3-SHA",
        "TLSv1.0",
        "TLSv1.1",
    ];

    println!("Testing TLS on {}:{}", host, port);

    let timeout = Duration::from_secs(5);

    let mut weak_found: Vec<String> = Vec::new();

    for item in &items {
        match try_connect_with_cipher(host, port, item, timeout) {
            Ok(Some(success)) => {
                println!("[OPEN]  {}", success);
                weak_found.push(success);
            }
            Ok(None) => {
                // Shouldn't normally happen; treat as not vulnerable
            }
            Err(err) => {
                // Print a short message for visibility, but not treat as success
                println!("[FAIL]  {} => {}", item, err);
            }
        }
    }

    if !weak_found.is_empty() {
        println!("\n[VULNERABLE] The following allowed/accepted weak items were detected:");
        for w in weak_found {
            println!("- {}", w);
        }
    } else {
        println!("\nNo obvious weak ciphers/protocols detected by these checks.");
    }

    Ok(())
}
