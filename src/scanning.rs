use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;

pub fn scan(address: &str, ports: &[u16], connect_timeout: Duration) -> anyhow::Result<Vec<u16>> {
    let mut open_ports: Vec<u16> = Vec::new();
    for port in ports.iter() {
        let target = format!("{}:{}", address, port);
        let target = SocketAddr::from_str(&target)?;
        match TcpStream::connect_timeout(&target, connect_timeout) {
            Ok(_) => open_ports.push(*port),
            Err(_) => (),
        }
    }

    Ok(open_ports)
}
