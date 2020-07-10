use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;
use std::{error, str};

use crate::{DEFAULT_CONNECT_TIMEOUT, READ_CONNECT_TIMEOUT, WRITE_CONNECT_TIMEOUT};

/// Grab banner of given target.
///
/// # Examples
///
/// Try to grab banner of service running on localhost:80
///
/// ```no_run
/// use polonium::grabbing::grab_banner;
/// let banner = grab_banner("127.0.0.1:80", &None, &None, &None);
/// ```
pub fn grab_banner(
    address: &str,
    connect_timeout: &Option<Duration>,
    read_timeout: &Option<Duration>,
    write_timeout: &Option<Duration>,
) -> Result<String, Box<dyn error::Error>> {
    let address = SocketAddr::from_str(address)?;

    let mut stream =
        TcpStream::connect_timeout(&address, connect_timeout.unwrap_or(DEFAULT_CONNECT_TIMEOUT))?;
    stream.set_read_timeout(Option::from(read_timeout.unwrap_or(READ_CONNECT_TIMEOUT)))?;
    stream.set_write_timeout(Option::from(write_timeout.unwrap_or(WRITE_CONNECT_TIMEOUT)))?;

    let mut buffer = [0; 512];

    // Try to read banner right after connecting
    let result = stream.read_exact(&mut buffer);
    if result.is_ok() {
        return Ok(String::from(str::from_utf8(&buffer)?));
    }

    // If timeout related error happens, do not fails
    // because we may need to talk first
    let error = result.err().unwrap();
    if error.kind() != ErrorKind::WouldBlock {
        return Err(error.into());
    }

    // If nothing was returned, send a dummy request
    stream.write_all("HEAD / HTTP/1.1\n\n".as_ref())?;

    // Try to read again
    stream.read_exact(&mut buffer)?;
    Ok(String::from(str::from_utf8(&buffer)?))
}
