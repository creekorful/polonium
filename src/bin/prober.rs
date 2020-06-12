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


    let banner = boron::probing::grab_banner(target, connect_timeout, read_timeout, write_timeout)
        .expect("unable to grab banner");
    print!("{}", banner);

    Ok(())
}