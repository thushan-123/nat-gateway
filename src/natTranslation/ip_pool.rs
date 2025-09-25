mod ip_handler;

use std::net::{Ipv4Addr, TcpStream};
use std::process::exit;

pub fn nat_pool(public_ip_range: &str, port_range: (i32, i32)){

    // public ip string convert and validate ipv4 address
    let ip: Ipv4Addr = public_ip_range.parse().expect(
        "Invalid ip address"
    );

    let (start_port, end_port) = port_range;
    if(
        !(
            (start_port <=65535 && start_port >=40000) &&
                (start_port <=65535 && start_port >=40000) &&
                (start_port < end_port)
            )
    ){
        eprintln!("Invalid prot range");
        exit(1);
    }

}