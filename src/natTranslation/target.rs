
use std::net::{TcpStream, Ipv4Addr, IpAddr, SocketAddr, ToSocketAddrs};

pub fn connect_to_target_with_kernal(
    public_ip: Ipv4Addr,
    public_port:u16,
    target: &str,
    request_data: String
) {
    let source_addr = SocketAddr::new(IpAddr::V4(public_ip), public_port);

    // resolve target domain
    let mut address_resolve = target.to_socket_addrs()?;
    let target_address = address_resolve
        .next()
        .ok_or("No addresses resolved for target")?;

    println!("Connecting from {} to {}", source_addr, target_address);

    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
    socket.bind(&source_addr.into())?;

    socket.set_read_timeout(Some(Duration::from_secs(10)))?;
    socket.set_write_timeout(Some(Duration::from_secs(10)))?;

    socket.connect(&target_address.into())?;

    let mut stream: std::net::TcpStream = socket.into();

    //  request data -> dest
    stream.write_all(request_data.as_bytes())?;
    stream.flush()?;


    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;

    let response = String::from_utf8_lossy(&buffer).to_string();
    return response;

}