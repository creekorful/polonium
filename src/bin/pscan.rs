use std::process;

use clap::{App, Arg};

use polonium::{parse_duration, parse_ports};

fn main() {
    let matches = App::new("Polonium Scanner (pscan)")
        .author("Aloïs Micard <alois@micard.lu>")
        .about("Scan given target to search for open ports")
        .arg(
            Arg::with_name("address")
                .required(true)
                .value_name("ADDRESS")
                .help("address of the target to be scanned"),
        )
        .arg(
            Arg::with_name("ports")
                .required(true)
                .value_name("PORT(S)")
                .help("set the ports to be scanned"),
        )
        .arg(
            Arg::with_name("connect-timeout")
                .long("connect-timeout")
                .value_name("VALUE")
                .default_value("500")
                .help("connect timeout (ms)"),
        )
        .get_matches();

    let address = matches.value_of("address").unwrap();
    let port_args = matches.value_of("ports").unwrap();
    let ports = match parse_ports(port_args) {
        Ok(ports) => ports,
        Err(e) => {
            eprintln!("error while parsing ports: {}", e);
            process::exit(1);
        }
    };

    let connect_timeout = parse_duration(&matches.value_of("connect-timeout"));

    println!(
        "Scanning target {} on following ports: {:?}",
        address, ports
    );

    let open_ports = match polonium::scanning::scan(address, &ports, &connect_timeout) {
        Ok(ports) => ports,
        Err(e) => {
            eprintln!("error while scanning target: {}", e);
            process::exit(1);
        }
    };

    if open_ports.is_empty() {
        println!("no open ports found.");
        return;
    }

    // TODO display closed port?
    for port in open_ports.iter() {
        println!("{}:{} OPEN", address, port)
    }
}
