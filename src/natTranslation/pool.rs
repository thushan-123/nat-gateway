use std;
use std::collections::HashMap;

pub struct AllocatedIpMap {
    allocated_ip_map: HashMap<String,String>
}


// socket - ((private_ip, private_port)  (public_ip, public_port))

// TcpStream { addr: 127.0.0.1:8080, peer: 127.0.0.1:49628, fd: 4 }

// 127.0.0.1:49628
// Accept the connection : 127.0.0.1:49628 TcpStream { addr: 127.0.0.1:8080, peer: 127.0.0.1:49628, fd: 4 }