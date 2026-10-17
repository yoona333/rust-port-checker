//! Utilities for parsing TCP ports and testing TCP connectivity.

use std::collections::BTreeSet;
use std::io;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

/// Parse a comma-separated list of ports and inclusive ranges.
///
/// Examples: `80`, `22,80,443`, `8000-8003,8080`.
/// Duplicate ports are removed and results are sorted.
pub fn parse_ports(input: &str) -> Result<Vec<u16>, String> {
    let mut ports = BTreeSet::new();

    for token in input.split(',') {
        let token = token.trim();
        if token.is_empty() {
            return Err("port list contains an empty item".to_string());
        }

        let (start, end) = match token.split_once('-') {
            Some((start, end)) => (parse_port(start)?, parse_port(end)?),
            None => {
                let port = parse_port(token)?;
                (port, port)
            }
        };

        if start > end {
            return Err(format!("invalid range '{token}': start exceeds end"));
        }

        for port in start..=end {
            ports.insert(port);
            if ports.len() > 1024 {
                return Err("too many ports (maximum: 1024)".to_string());
            }
        }
    }

    Ok(ports.into_iter().collect())
}

fn parse_port(raw: &str) -> Result<u16, String> {
    let port = raw
        .trim()
        .parse::<u16>()
        .map_err(|_| format!("invalid TCP port: '{}'", raw.trim()))?;

    if port == 0 {
        return Err("TCP port must be between 1 and 65535".to_string());
    }

    Ok(port)
}

/// Return `true` if a TCP connection to any resolved address succeeds.
///
/// DNS errors are returned to the caller. A refused or timed-out TCP
/// connection is reported as `Ok(false)`.
pub fn is_port_open(host: &str, port: u16, timeout: Duration) -> io::Result<bool> {
    let addresses = (host, port).to_socket_addrs()?;
    let mut any_address = false;

    for address in addresses {
        any_address = true;
        if TcpStream::connect_timeout(&address, timeout).is_ok() {
            return Ok(true);
        }
    }

    if any_address {
        Ok(false)
    } else {
        Err(io::Error::new(
            io::ErrorKind::AddrNotAvailable,
            "host resolved to no socket addresses",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{is_port_open, parse_ports};
    use std::net::TcpListener;
    use std::time::Duration;

    #[test]
    fn parses_single_port() {
        assert_eq!(parse_ports("443").unwrap(), vec![443]);
    }

    #[test]
    fn parses_ranges_and_removes_duplicates() {
        assert_eq!(
            parse_ports("443,80,8000-8002,80").unwrap(),
            vec![80, 443, 8000, 8001, 8002]
        );
    }

    #[test]
    fn rejects_invalid_ports() {
        for invalid in ["", "0", "65536", "abc", "22,", "100-99", "1-1025"] {
            assert!(parse_ports(invalid).is_err(), "should reject {invalid}");
        }
    }

    #[test]
    fn connects_to_a_local_listening_port() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        assert!(is_port_open("127.0.0.1", port, Duration::from_millis(500)).unwrap());
    }
}
