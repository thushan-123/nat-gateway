use std::io::{Read,Write,BufRead};
use std::net::{TcpStream, TcpListener};

fn handle_client(mut stream : TcpStream){
    let mut buffer = [0;4096];
    
    stream.read(&mut buffer).expect(
        "fail to read data from client"
    );
}


fn main() {
    println!("Hello, world!");
}
