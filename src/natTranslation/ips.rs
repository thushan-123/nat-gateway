
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
}