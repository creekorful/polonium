use std::process;

use clap::{App, Arg};

use boron::parse_duration;

fn main() {
    let matches = App::new("Grabber")
        .version("0.0.1")
        .author("Aloïs Micard <alois@micard.lu>")
        .about("Grab given address to gather banner details")
        .arg(
            Arg::with_name("address")
                .required(true)
                .value_name("ADDRESS:PORT")
                .help("address of the service to be scanned"),
        )
        .arg(
            Arg::with_name("connect-timeout")
                .long("connect-timeout")
                .value_name("VALUE")
                .default_value("1000")
                .help("connect timeout (ms)"),
        )
        .arg(
            Arg::with_name("read-timeout")
                .long("read-timeout")
                .value_name("VALUE")
                .default_value("500")
                .help("read timeout (ms)"),
        )
        .arg(
            Arg::with_name("write-timeout")
                .long("write-timeout")
                .value_name("VALUE")
                .default_value("500")
                .help("write timeout (ms)"),
        )
        .get_matches();

    let target = matches.value_of("address").unwrap();

    let connect_timeout = parse_duration(&matches.value_of("connect-timeout"));
    let read_timeout = parse_duration(&matches.value_of("read-timeout"));
    let write_timeout = parse_duration(&matches.value_of("write-timeout"));

    let banner =
        match boron::grabbing::grab_banner(target, &connect_timeout, &read_timeout, &write_timeout)
        {
            Ok(banner) => banner,
            Err(e) => {
                eprintln!("Error while grabbing banner: {}", e);
                process::exit(1);
            }
        };

    println!("{}", banner);
}
