use std::os::unix::io::AsRawFd;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::process::Command;


pub fn entry() {
    run_multiple_processes();
    command_example();
}

pub fn run_multiple_processes() {
    let stdin_fd = std::io::stdin().as_raw_fd();
    println!("STDIN file descriptor: {}", stdin_fd);

    let stdprc = process::id();
    println!("Process ID: {}", stdprc);

    // Atomic boolean to indicate whether the program should continue running
    let running = Arc::new(AtomicBool::new(true));

    // Clone the running flag for the signal handler thread
    let running_clone = Arc::clone(&running);

    // Spawn a thread to handle signals
    thread::spawn(move || {
        // Handle SIGINT (Ctrl+C)
        ctrlc::set_handler(move || {
            running_clone.store(false, Ordering::SeqCst);
        })
            .expect("Error setting Ctrl-C handler");
    });

    println!("Press Ctrl+C to exit...");

    // Main loop
    while running.load(Ordering::SeqCst) {
        // Do some work
        println!("Running...");
        //then sleep a bit
        thread::sleep(Duration::from_secs(1));
    }

    // got CTRL+C
    println!("Exiting...");
}


fn command_example() {
    let output =Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
        panic!("FAILED TO EXECUTE PROCESS: {}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }
}
