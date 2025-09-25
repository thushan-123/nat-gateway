
use std::collections::HashMap;
use std::net::{Ipv4Addr, TcpStream};
use std::rand::{task_rng, Rng};
struct AssignIP{
    assign_ip: HashMap<u16,(Ipv4Addr, bool)>,
    num_of_ip: u8,
}

impl AssignIP{
    pub fn new() ->self {
        AssignIP{
            assign_ip: HashMap::new(),
            num_of_ip : u8 = 100,
        }
    }

    pub fn insert_ip(&mut self, start_public_ip :&str ,subnet_mask:&str, num_of_ip: u8){

        let ip: Ipv4Addr = public_ip.parse().expect(
            "Invalid ip address"
        );

        self.num_of_ip = self.num_of_ip;

        let mut ip_u32 = u32::form(ip);

        for i in 1..num_of_ip {
            let mut new_ip = Ipv4Addr::from(ip_u32 + i as u32);
            self.assign_ip.insert(new_ip, false);
        }

    }

    pub fn get_ip(&mut self) -> (Ipv4Addr, bool, u8) {
        let mut rng = rand::task_rng();
        let random_number: u8 = rng.gen_range(0,self.num_of_ip);

        let (ip, status) = self.assign_ip.get(random_number);
        if(!status){
            self.assign_ip[random_number] = (ip, true);
        }
        return (ip, status, random_number) ;
    }

    pub fn barrow_ip(&mut self, id: u8, ip: Ipv4Addr){
        let (ip, status) = self.assign_ip.get(id);
        self.assign_ip[id] = (ip , false);
    }
}
