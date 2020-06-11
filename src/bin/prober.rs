use std::{env, io};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // TODO: use CLI framework instead
    let args: Vec<String> = env::args().collect();
    let target = args.get(1).expect("target is required");

    let banner = grab_banner(target)?;
    print!("{}", banner);

    Ok(())
}

fn grab_banner(svc: &str) -> io::Result<String> {
    // TODO: use connect_timeout instead
    let mut stream = TcpStream::connect(svc).expect("unable to connect to target");
    stream.set_read_timeout(Option::from(Duration::from_millis(500))).expect("unable to set_read_timeout");

    let mut buffer = [0; 512];

    // Try to read banner right after connecting
    if stream.read(&mut buffer).is_ok() {
        return Ok(String::from(str::from_utf8(&buffer).unwrap()));
    }

    // If nothing was returned, send a dummy request
    stream.write("HEAD / HTTP/1.1\n\n".as_ref())?;

    // Try to read again
    stream.read(&mut buffer)?;
    return Ok(String::from(str::from_utf8(&buffer).unwrap()));
}