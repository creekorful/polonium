use std::time::Duration;

fn main() {
    let ports = [80, 8080, 3000];
    let address = "127.0.0.1";

    let open_ports = boron::scanning::scan(address, &ports, Duration::from_millis(500)).unwrap();
    for port in open_ports.iter() {
        println!("{}:{} OPEN", address, port)
    }
}