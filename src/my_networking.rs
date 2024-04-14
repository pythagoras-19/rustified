use std::io::{Read, Write};
use std::net::IpAddr;
use std::net:: {TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use crate::my_networking;

pub fn access() {
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}

pub fn start_server_and_client_threads() {
    // start server and new thread
    let server_thread = thread::spawn( || {
        let result = my_networking::create_tcp_listener(8888);
        match result {
            Ok(_) => println!("Created TCP listener at port 8888"),
            Err(e) => panic!("Error creating the TCP listener: {}", e)
        };
    });

    // wait for server to start
    thread::sleep(Duration::from_secs(1));

    let client_thread = thread::spawn(|| {
        let result = my_networking:: create_tcp_stream("localhost:8888");
        match result {
            Ok(_) => println!("Created a TCP stream to localhost:8888"),
            Err(e) => panic!("Error creating the TCP stream: {}", e),
        };
    });

    //wait for server and client threads to finish up
    server_thread.join().unwrap();
    client_thread.join().unwrap();
}

pub fn create_tcp_listener(port: u16) -> std::io::Result<()> {
    let listener = TcpListener::bind(("localhost", port))?;
    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                stream
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(e);
            }
        };

        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => {
                let msg = String::from_utf8_lossy(&buffer[..]);
                println!("Received: {}", msg);
            },
            Err(e) => println!("Failed to read from socket {}", e),
        }
    }
    Ok(())
}

pub fn create_tcp_stream(address: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(address)?;
    stream.write_all(b"Hello, server!")?;
    Ok(())
}