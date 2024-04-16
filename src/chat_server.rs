use std::io::{Read, Write, Error};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const LOCATION: &str = "localhost:9999";

pub fn start() {
    let server_thread = thread::spawn(|| {
        start_server(LOCATION).expect("TODO: panic message");
    });

    let client_thread = thread::spawn(|| {
        // Sleep for a bit to ensure the server has time to start
        thread::sleep(Duration::from_secs(1));
        start_client(LOCATION).expect("TODO: panic message");
    });

    // Optionally, wait for both threads to finish
    server_thread.join().unwrap();
    client_thread.join().unwrap();
}
// Server
fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) -> Result<(), Error> {
    println!("New client: {}", stream.peer_addr()?);
    let mut buf = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        let msg = String::from_utf8(buf[..bytes_read].to_vec()).unwrap();
        println!("Received: {}", msg);
        for client in clients.lock().unwrap().iter_mut() {
            client.write_all(msg.as_bytes())?;
        }
    }
}

pub(crate) fn start_server(addr: &str) -> Result<(), Error> {
    println!("Starting server...");
    let listener = TcpListener::bind(addr)?;
    let clients = Arc::new(Mutex::new(Vec::new()));
    for stream in listener.incoming() {
        let stream = stream?;
        clients.lock().unwrap().push(stream.try_clone()?);
        let clients = Arc::clone(&clients);
        thread::spawn(move || {
            handle_client(stream, clients).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
    }
    Ok(())
}

// Client
fn handle_server(mut stream: TcpStream) -> Result<(), Error> {
    let mut buf = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        print!("{}", String::from_utf8(buf[..bytes_read].to_vec()).unwrap());
    }
}

pub(crate) fn start_client(addr: &str) -> Result<(), Error> {
    println!("Starting client...");
    let stream = TcpStream::connect(addr)?;
    thread::spawn(move || {
        handle_server(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
    });
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        let mut stream = TcpStream::connect(addr)?;
        stream.write_all(buf.as_bytes())?;
    }
}