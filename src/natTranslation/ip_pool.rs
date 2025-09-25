mod ip_handler;

use std::net::{Ipv4Addr, TcpStream};
use std::process::exit;
use std::rand::{task_rng, Rng};
pub fn nat_pool(public_ip_range: &str, port_range: (u32, u32)){

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

    let mut rng = rand::task_rng();
    let random_port: u32 = rng.gen_range(start_port,end_port);

}