use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let target = args.get(1).expect("target is required");

    let mut stream = TcpStream::connect(target)?;
    stream.write("HEAD / HTTP/1.1\n\n".as_ref())?;

    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    print!("{}", buffer);

    Ok(())
}
