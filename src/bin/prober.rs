use std::env;
use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::str;
use std::str::FromStr;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // TODO: use CLI framework instead
    let args: Vec<String> = env::args().collect();

    let target = args.get(1).expect("target is required");
    let connect_timeout = Duration::from_millis(1000);
    let read_timeout = Duration::from_millis(500);


    let banner = grab_banner(target, connect_timeout, read_timeout).expect("unable to grab banner");
    print!("{}", banner);

    Ok(())
}

fn grab_banner(address: &str, connect_timeout: Duration, read_timeout: Duration) -> Result<String, failure::Error> {
    let address = SocketAddr::from_str(address)?;

    let mut stream = TcpStream::connect_timeout(&address, connect_timeout)?;
    stream.set_read_timeout(Option::from(read_timeout))?;

    let mut buffer = [0; 512];

    // Try to read banner right after connecting
    let result = stream.read(&mut buffer);
    if result.is_ok() {
        return Ok(String::from(str::from_utf8(&buffer)?));
    }

    let error = result.err().unwrap();
    if error.kind() != ErrorKind::WouldBlock {
        return Err(failure::err_msg(error));
    }

    // If nothing was returned, send a dummy request
    stream.write("HEAD / HTTP/1.1\n\n".as_ref())?;

    // Try to read again
    stream.read(&mut buffer)?;
    return Ok(String::from(str::from_utf8(&buffer)?));
}