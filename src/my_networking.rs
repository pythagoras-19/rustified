use std::io::{Read, Write};
use std::net:: {TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use serde:: {Serialize, Deserialize};
use indicatif:: {ProgressBar, ProgressStyle};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    message: String,
    number: i32,
}

pub fn start_server_and_client_threads() {
    println!("Starting server...");
    //progress bar
    let pb = ProgressBar::new(20);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{bar:40.cyan/blue}] ({eta}) ")
        .unwrap().progress_chars("#>-"));

    // start pb thread
    let progress_thread = thread::spawn(move || {
        for _ in 0..10 {
            pb.inc(1);
            thread::sleep(Duration::from_millis(500));
        }
        pb.finish_with_message("Server successfully started.");
    });

    // start server and new thread
    let server_thread = thread::spawn( || {
        let result = create_tcp_listener(8888);
        match result {
            Ok(_) => println!("Created TCP listener at port 8888"),
            Err(e) => panic!("Error creating the TCP listener: {}", e)
        };
    });

    // wait for pb to finish
    progress_thread.join().unwrap();

    // wait for server to start
    thread::sleep(Duration::from_secs(1));

    let client_thread = thread::spawn(|| {
        let result = create_tcp_stream("localhost:8888");
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
            Ok(bytes_read) => {
                let json_data = String::from_utf8_lossy(&buffer[..bytes_read]);
                let data : Data = serde_json::from_str(&json_data).unwrap();
                println!("Received: {:?}", data);
            },
            Err(e) => println!("Failed to read from socket {}", e),
        }
    }
    Ok(())
}

pub fn create_tcp_stream(address: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(address)?;

    let data = Data {
        message: "Hello Server!!".to_string(),
        number: 42,
    };

    let json_data = serde_json::to_string(&data).unwrap(); // serial the data
    stream.write_all(json_data.as_bytes())?; // send the json over the tcp stream

    Ok(())
}