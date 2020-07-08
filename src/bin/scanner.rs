use std::process;
use std::time::Duration;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Scanner")
        .version("0.0.1")
        .author("Aloïs Micard <alois@micard.lu>")
        .about("Scan given target to search for open ports")
        .arg(
            Arg::with_name("target")
                .required(true)
                .value_name("TARGET")
                .help("address of the target to be scanned"),
        )
        .arg(
            Arg::with_name("ports")
                .required(true)
                .long("ports")
                .value_name("PORT(S)")
                .help("Set the ports to be scanned"),
        )
        .arg(
            Arg::with_name("connect-timeout")
                .long("connect-timeout")
                .value_name("VALUE")
                .default_value("500")
                .help("connect timeout (ms)"),
        )
        .get_matches();

    let target = matches.value_of("target").unwrap();

    let port_args = matches.value_of("ports").unwrap();
    let ports = extract_ports(port_args).unwrap();

    let connect_timeout = matches.value_of("connect-timeout").unwrap();
    let connect_timeout = connect_timeout.parse::<u64>().unwrap();
    let connect_timeout = Duration::from_millis(connect_timeout);

    println!("Scanning target {} on following ports: {:?}", target, ports);

    // TODO allow parallel scan?
    let open_ports = match boron::scanning::scan(target, &ports, connect_timeout) {
        Ok(ports) => ports,
        Err(e) => {
            eprintln!("Error while scanning target: {}", e);
            process::exit(1);
        }
    };

    // TODO display closed port?
    for port in open_ports.iter() {
        println!("{}:{} OPEN", target, port)
    }
}

fn extract_ports(ports_arg: &str) -> anyhow::Result<Vec<u16>> {
    let mut ports: Vec<u16> = Vec::new();

    // single port value
    let port = ports_arg.parse::<u16>();
    if let Ok(port) = port {
        ports.push(port);
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

    Err(anyhow::anyhow!("Unable to extract ports"))
}
