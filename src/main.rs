use rust_port_checker::{is_port_open, parse_ports};
use std::env;
use std::process;
use std::time::Duration;

const USAGE: &str = "rust-port-checker — a lightweight TCP connectivity checker

Usage:
  rust-port-checker <host> <ports> [--timeout-ms <milliseconds>]
  rust-port-checker --help

Examples:
  rust-port-checker example.com 80,443
  rust-port-checker 127.0.0.1 3000-3003 --timeout-ms 1000

Ports: comma-separated values and inclusive ranges (1-65535, up to 1024 ports).
Exit codes: 0 = all checked ports open, 1 = at least one port closed, 2 = error.
Only check hosts that you own or are authorized to test.";

fn run() -> Result<bool, String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 1 && (args[0] == "--help" || args[0] == "-h") {
        println!("{USAGE}");
        return Ok(true);
    }

    if args.len() != 2 && args.len() != 4 {
        return Err(format!("invalid arguments\n\n{USAGE}"));
    }

    let host = args[0].trim();
    if host.is_empty() {
        return Err("host cannot be empty".to_string());
    }

    let ports = parse_ports(&args[1])?;
    let timeout_ms: u64 = if args.len() == 4 {
        if args[2] != "--timeout-ms" {
            return Err(format!("unknown option: {}", args[2]));
        }
        args[3]
            .parse()
            .map_err(|_| "timeout must be a number of milliseconds".to_string())?
    } else {
        750
    };
    if !(1..=60_000).contains(&timeout_ms) {
        return Err("timeout must be between 1 and 60000 milliseconds".to_string());
    }

    let timeout = Duration::from_millis(timeout_ms);
    let mut all_open = true;
    for port in ports {
        match is_port_open(host, port, timeout) {
            Ok(true) => println!("OPEN    {host}:{port}"),
            Ok(false) => {
                println!("CLOSED  {host}:{port}");
                all_open = false;
            }
            Err(error) => return Err(format!("cannot check {host}:{port}: {error}")),
        }
    }
    Ok(all_open)
}

fn main() {
    match run() {
        Ok(true) => {}
        Ok(false) => process::exit(1),
        Err(error) => {
            eprintln!("Error: {error}");
            process::exit(2);
        }
    }
}
