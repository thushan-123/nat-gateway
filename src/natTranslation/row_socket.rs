use std::net::Ipv4Addr;
use pnet::packet::tcp::{MutableTcpPacket, TcpFlags};
use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::transport::{transport_channel, TransportChannelType::Layer3, TransportReceiver, TransportSender};
use pnet::packet::Packet;

fn raw_socket_bypass_kernel(
    public_ip: Ipv4Addr,
    public_port: u16,
    target: &str,
    request_data: &str,
) {
    //create raw socket
    let (mut tx, mut rx) = transport_channel(4096, Layer3(IpNextHeaderProtocols::Tcp))
        .expect("Failed to create raw socket");

    // build ipv4 header packet
    let mut ipv4_buffer = [0u8; 40];
    let mut ip_packet = MutableIpv4Packet::new(&mut ipv4_buffer).unwrap();
    ip_packet.set_source(public_ip);
    ip_packet.set_destination(target_ip);
    ip_packet.set_next_level_protocol(IpNextHeaderProtocols::Tcp);
    ip_packet.set_version(4);
    ip_packet.set_header_length(5);
    ip_packet.set_ttl(64);

    // tcp header build
    let mut tcp_buffer = [0u8; 20];
    let mut tcp_packet = MutableTcpPacket::new(&mut tcp_buffer).unwrap();
    tcp_packet.set_source(public_port);
    tcp_packet.set_destination(target_port);
    tcp_packet.set_flags(TcpFlags::SYN);
    tcp_packet.set_sequence(0);

    ip_packet.set_payload(tcp_packet.packet());

    // Send SYN
    tx.send_to(ip_packet, std::net::IpAddr::V4(target_ip))
        .expect("Failed to send SYN");

    println!("SYN sent. You would now need to wait for SYN-ACK and complete handshake manually.");

    // response requires capture packets
    let mut recv_buf = [0u8; 4096];
    match rx.next() {
        Ok(packet) => {
            println!("Received packet of length: {}", packet.len());
        }
        Err(e) => eprintln!("Error receiving packet: {:?}", e),
    }

}

