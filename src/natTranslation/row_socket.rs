use std::net::{Ipv4Addr, ToSocketAddrs};
use pnet::packet::tcp::{MutableTcpPacket, TcpFlags};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::transport::{transport_channel, TransportChannelType::Layer3};

fn raw_socket_bypass_kernel(
    public_ip: Ipv4Addr,
    public_port: u16,
    target: &str,
    target_port: u16,
    request_data: &str,
) {
    // resolve target domain to IP
    let target_ip: Ipv4Addr = match (target, 0).to_socket_addrs() {
        Ok(mut addrs) => match addrs.next() {
            Some(std::net::SocketAddr::V4(addr)) => *addr.ip(),
            _ => panic!("Could not resolve target IP"),
        },
        Err(e) => panic!("DNS resolution failed: {:?}", e),
    };

    // create raw socket
    let (mut tx, mut rx) = transport_channel(4096, Layer3(IpNextHeaderProtocols::Tcp))
        .expect("Failed to create raw socket");

    // Build IPv4 header
    let mut ipv4_buffer = [0u8; 40];
    let mut ip_packet = MutableIpv4Packet::new(&mut ipv4_buffer).unwrap();
    ip_packet.set_source(public_ip);
    ip_packet.set_destination(target_ip);
    ip_packet.set_next_level_protocol(IpNextHeaderProtocols::Tcp);
    ip_packet.set_version(4);
    ip_packet.set_header_length(5);
    ip_packet.set_ttl(64);

    // build TCP header (SYN)
    let mut tcp_buffer = [0u8; 20];
    let mut tcp_packet = MutableTcpPacket::new(&mut tcp_buffer).unwrap();
    tcp_packet.set_source(public_port);
    tcp_packet.set_destination(target_port);
    tcp_packet.set_flags(TcpFlags::SYN);
    tcp_packet.set_sequence(0);

    // attach TCP to IP data
    ip_packet.set_payload(tcp_packet.packet());

    // send SYN
    tx.send_to(ip_packet, std::net::IpAddr::V4(target_ip))
        .expect("Failed to send SYN");
    println!("SYN sent to {}:{}", target_ip, target_port);

    // Capture a response (SYN-ACK)
    let mut recv_buf = [0u8; 4096];
    match rx.next() {
        Ok(packet) => {
            println!("Received packet of length: {}", packet.len());

        }
        Err(e) => eprintln!("Error receiving packet: {:?}", e),
    }

}


