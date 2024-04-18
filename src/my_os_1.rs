use nix::sys::signal::{self, Signal, SigHandler, SigAction, SaFlags};
use std::sync::atomic::{AtomicBool, Ordering};


static SHOULD_TERMINATE: AtomicBool = AtomicBool::new(false);

// ffi with C
extern "C" fn handle_sigint(_:i32) {
    SHOULD_TERMINATE.store(true, Ordering::SeqCst);
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

    // main loop
    while !SHOULD_TERMINATE.load(Ordering::SeqCst) {

        //todo: do some werk

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("Shutting down os 1...");

    Ok(())
}