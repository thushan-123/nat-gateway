use std::io::{Read,Write};
use std::net::{TcpStream, TcpListener};

fn handle_client(mut stream : TcpStream){
    let mut buffer = [0;4096];
    
    stream.read(&mut buffer).expect(
        "fail to read data from client"
    );
    
    // convert data buffer into string
    
    let request = String::from_utf8_lossy(&buffer[..]); // decode buff data
    
    println!("Received request : {}", request);  // print the request

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHii hello client..".as_bytes();

    stream.write(response).expect(
        "Fail to send response to client"
    );
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect(
        "fail to bind address 127.0.0.1:8080"
    );
    println!("TCP server running on 127.0.0.1:8080");
    
    loop {
        match listener.accept() {
            Ok((socket,addr)) => {
                println!("Accept the connection : {}", addr);
                
                // create a new separate thread
                std::thread::spawn(|| {
                    handle_client(socket);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection : {}", e);
            }
        }
        
    }
    
}
