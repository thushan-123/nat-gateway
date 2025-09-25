
use std::collections::HashMap;
use std::net::{Ipv4Addr, TcpStream};

struct AssignIP{
    assign_ip: HashMap<Ipv4Addr, bool>
}

impl AssignIP{
    pub fn new() ->self {
        AssignIP{
            assign_ip: HashMap::new(),
        }
    }

    pub fn insert_ip(&mut self, start_public_ip :&str ,subnet_mask:&str, num_of_ip: u8){

        let ip: Ipv4Addr = public_ip.parse().expect(
            "Invalid ip address"
        );

        let mut ip_u32 = u32::form(ip);

        for i in 1..num_of_ip {
            let mut new_ip = Ipv4Addr::from(ip_u32 + i as u32);
            self.assign_ip.insert(new_ip, false);
        }



    }
}