
use std::net::{TcpStream, Ipv4Addr, IpAddr, SocketAddr};

pub fn connect_to_target_with_kernal(
    public_ip: Ipv4Addr,
    public_port:u16
) -> Result<()>{
    let translate_source_ip = IpAddr::V4(public_ip);
    let translate_source_port = public_port;

    let translate_source_address = SocketAddr::new(
        translate_source_ip,
        translate_source_port
    );


}