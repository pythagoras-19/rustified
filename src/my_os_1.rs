// to handle UNIX signals
use nix::sys::signal::{self, Signal, SigHandler, SigAction, SaFlags};
use std::sync::atomic::{AtomicBool, Ordering};
use indicatif::*;


// to coordinate between the signal handler and the main application logic
static SHOULD_TERMINATE: AtomicBool = AtomicBool::new(false); // can be shared amongst mult threads

// ffi with C
extern "C" fn handle_sigint(_:i32) {
    SHOULD_TERMINATE.store(true, Ordering::SeqCst); // SeqCst = "Sequential Consistency"
    /*
    Memory ordering: crucial to concurrent programming, particularly when using atomic variables to coordinate
    between threads.

    SeqCst is one of the memory ordering options available in atomic operations.
    It is part of the std::sync::atomic library
     */
    println!();
    println!("Caught SIGINT!");
}


pub fn entry() {
    my_os_1_main().expect("TODO: panic message");
}

fn my_os_1_main() -> Result<(), nix::Error> {
    // define the action to take on SIGINT
    let sig_action = SigAction::new(
        SigHandler::Handler(handle_sigint),
        SaFlags::SA_RESTART,
        signal::SigSet::empty(),
    );

    unsafe {
        signal::sigaction(Signal::SIGINT, &sig_action)?;
    }

    let mut time = 0;
    let bar = ProgressBar::new(10);
    // main loop
    while !SHOULD_TERMINATE.load(Ordering::SeqCst) {
        time += 1;
        for i in 0..18 {
            println!("...{}", i);
        }
        println!("Time: {}", time);
        bar.inc(1);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    bar.finish();

    println!("Shutting down os 1...");

    Ok(())
}