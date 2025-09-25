use std;
use std::collections::HashMap;
use std::net::{TcpStream};

pub struct AllocatedIpMap {
    allocated_ip_map: HashMap<TcpStream,String>,
}

impl AllocatedIpMap{
    pub fn new() -> self{
        AllocatedIpMap {
            allocated_ip_map: HashMap::new(),
        }
    }

    pub fn allocate_ip(
        &mut self,
        socket : TcpStream,
        private_ip: &str,
        private_port: i32,
        public_ip: &str,
        public_port: i32
    ){
        let data_tuple = (
            (
                private_ip.to_string(),   // private ip tuple
                private_port.to_string(),
                ),
            (
                public_ip.to_string(),    // public ip tuple
                private_port.to_string()
                )
            );

        self.allocated_ip_map.insert(
            socket, data_tuple
        )
    }

    pub fn remove_stream(&mut self, socket: TcpStream){
        self.allocated_ip_map.remove(socket);
    }
}
// socket - ((private_ip, private_port)  (public_ip, public_port))

// TcpStream { addr: 127.0.0.1:8080, peer: 127.0.0.1:49628, fd: 4 }

// 127.0.0.1:49628
// Accept the connection : 127.0.0.1:49628 TcpStream { addr: 127.0.0.1:8080, peer: 127.0.0.1:49628, fd: 4 }