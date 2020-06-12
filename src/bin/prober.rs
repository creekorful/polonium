use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::str;
use std::str::FromStr;
use std::time::Duration;

use clap::{App, Arg};

fn main() -> std::io::Result<()> {
    let matches = App::new("Prober")
        .version("0.0.1")
        .author("Aloïs Micard <alois@micard.lu>")
        .about("Probe given address to gather banner details")
        .arg(Arg::with_name("address")
            .required(true)
            .value_name("ADDRESS:PORT")
            .help("address of the service to be scanned"))
        .arg(Arg::with_name("connect-timeout")
            .long("connect-timeout")
            .value_name("VALUE")
            .default_value("1000")
            .help("connect timeout (ms)"))
        .arg(Arg::with_name("read-timeout")
            .long("read-timeout")
            .value_name("VALUE")
            .default_value("500")
            .help("read timeout (ms)"))
        .arg(Arg::with_name("write-timeout")
            .long("write-timeout")
            .value_name("VALUE")
            .default_value("500")
            .help("write timeout (ms)"))
        .get_matches();

    let target = matches.value_of("address").unwrap();

    let connect_timeout = matches.value_of("connect-timeout").unwrap();
    let connect_timeout = connect_timeout.parse::<u64>().unwrap();
    let connect_timeout = Duration::from_millis(connect_timeout);

    let read_timeout = matches.value_of("read-timeout").unwrap();
    let read_timeout = read_timeout.parse::<u64>().unwrap();
    let read_timeout = Duration::from_millis(read_timeout);

    let write_timeout = matches.value_of("write-timeout").unwrap();
    let write_timeout = write_timeout.parse::<u64>().unwrap();
    let write_timeout = Duration::from_millis(write_timeout);


    let banner = grab_banner(target, connect_timeout, read_timeout, write_timeout)
        .expect("unable to grab banner");
    print!("{}", banner);

    Ok(())
}

fn grab_banner(address: &str, connect_timeout: Duration,
               read_timeout: Duration, write_timeout: Duration) -> Result<String, failure::Error> {
    let address = SocketAddr::from_str(address)?;

    let mut stream = TcpStream::connect_timeout(&address, connect_timeout)?;
    stream.set_read_timeout(Option::from(read_timeout))?;
    stream.set_write_timeout(Option::from(write_timeout))?;

    let mut buffer = [0; 512];

    // Try to read banner right after connecting
    let result = stream.read(&mut buffer);
    if result.is_ok() {
        return Ok(String::from(str::from_utf8(&buffer)?));
    }

    // If timeout related error happens, do not fails
    // because we may need to talk first
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