use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;

pub mod grabbing;
pub mod scanning;

const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(1);
const READ_CONNECT_TIMEOUT: Duration = Duration::from_secs(1);
const WRITE_CONNECT_TIMEOUT: Duration = Duration::from_secs(1);

/// Parse a duration from given milliseconds string.
///
/// # Examples
///
/// ```
/// use polonium::parse_duration;
/// use std::time::Duration;
/// let duration = parse_duration(&Some("42"));
/// assert_eq!(duration, Some(Duration::from_millis(42)));
/// ```
pub fn parse_duration(value: &Option<&str>) -> Option<Duration> {
    value
        .map(|v| v.parse::<u64>())
        .filter(|v| v.is_ok())
        .map(|v| Duration::from_millis(v.unwrap()))
}

/// Perform the DNS resolution of given hostname (if any)
///
/// # Examples
///
/// ```no_run
/// use polonium::resolve;
/// use std::net::{SocketAddrV4, SocketAddr};
/// let addr = resolve("localhost:80");
/// assert_eq!(addr.unwrap(), SocketAddr::from(([127, 0, 0, 1], 80)));
/// ```
pub fn resolve(address: &str) -> Result<SocketAddr, std::io::Error> {
    address.to_socket_addrs().map(|i| i.as_slice()[0])
}

/// Function that parse ports from several formats
///
/// # Examples
///
/// ```
/// use polonium::parse_ports;
/// assert_eq!(parse_ports("80").unwrap(), [80]);
/// assert_eq!(parse_ports("80,443,8080").unwrap(), [80, 443, 8080]);
/// assert_eq!(parse_ports("20-22").unwrap(), [20, 21, 22]);
/// ```
pub fn parse_ports(ports_arg: &str) -> Result<Vec<u16>, Box<dyn error::Error>> {
    let mut ports: Vec<u16> = Vec::new();

    // single port value
    let port = ports_arg.parse::<u16>();
    if let Ok(port) = port {
        ports.push(port);
        return Ok(ports);
    }

    // file given
    if ports_arg.starts_with('@') {
        let path = ports_arg.replace('@', "");

        // read file line by line
        let ports: Vec<u16> = BufReader::new(File::open(path)?)
            .lines()
            .filter(|p| p.is_ok())
            .map(|p| p.unwrap().parse::<u16>().unwrap())
            .collect();
        return Ok(ports);
    }

    // multiple ports given (i.e 80,443,8080)
    if ports_arg.contains(',') {
        ports = ports_arg
            .split(',')
            .map(|p| p.parse::<u16>().unwrap())
            .collect();
        return Ok(ports);
    }

    // range given (i.e 80-3000)
    if ports_arg.contains('-') {
        let parts: Vec<&str> = ports_arg.split('-').collect();
        let start = parts[0].parse::<u16>()?;
        let end = parts[1].parse::<u16>()?;

        for i in start..end + 1 {
            ports.push(i);
        }

        return Ok(ports);
    }

    Err(format!("`{}` is not a valid format.", ports_arg).into())
}
