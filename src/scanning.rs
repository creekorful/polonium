use std::error;
use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;

use crate::DEFAULT_CONNECT_TIMEOUT;

/// Scan given address for open ports.
///
/// # Examples
///
/// Scan localhost on ports 80, 8080:
///
/// ```no_run
/// use polonium::scanning::scan;
/// let open_ports = scan("127.0.0.1", &[80, 8080], &None);
/// ```
pub fn scan(
    address: &str,
    ports: &[u16],
    connect_timeout: &Option<Duration>,
) -> Result<Vec<u16>, Box<dyn error::Error>> {
    let mut open_ports: Vec<u16> = Vec::new();
    for port in ports.iter() {
        let target = format!("{}:{}", address, port);
        let target = SocketAddr::from_str(&target)?;
        if TcpStream::connect_timeout(&target, connect_timeout.unwrap_or(DEFAULT_CONNECT_TIMEOUT))
            .is_ok()
        {
            open_ports.push(*port)
        }
    }

    Ok(open_ports)
}
