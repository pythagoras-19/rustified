use std::io::{Read, Write, Error};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{ AtomicBool, Ordering };
use std::time::Duration;
/**
    TODO: Bug: Cant exit with CTRL-C
**/
const LOCATION: &str = "localhost:9999";

pub fn entry() {
    start();
}

fn start() {
    println!("Starting chat server...");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let r_2 = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler!!");

    let server_thread = thread::spawn(move || {
        while running.load(Ordering::SeqCst) {
            start_server(LOCATION).expect("TODO: panic message");
        }
    });

    let client_thread = thread::spawn(move || {
        while r_2.load(Ordering::SeqCst) {
            // Sleep for a bit to ensure the server has time to start
            thread::sleep(Duration::from_secs(1));
            start_client(LOCATION).expect("TODO: panic message");
        }
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

fn start_server(addr: &str) -> Result<(), Error> {
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

fn handle_server(mut stream: TcpStream) -> Result<(), Error> {
    let mut buf = [0; 1024];
    loop {
        match stream.read(&mut buf) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // EOF, so break the loop
                    break;
                }
                print!("Message sent from: {:?} => {}", stream.peer_addr()?, String::from_utf8(buf[..bytes_read].to_vec()).unwrap());
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {:?}", e);
                return Err(e);
            }
        }
    }
    Ok(())
}

fn start_client(addr: &str) -> Result<(), Error> {
    println!("Starting client...");
    let mut stream = TcpStream::connect(addr)?;
    let stream_clone = stream.try_clone()?;
    thread::spawn(move || {
        handle_server(stream_clone).unwrap_or_else(|error| eprintln!("{:?}", error));
    });
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        stream.write_all(buf.as_bytes())?;
    }
}