// import VRC client input and output
mod client;
// import traits for VRC client
mod traits;
// import CLI functions
mod cli;

// import logging features
use log::{debug, info, warn, error};

// import threading capabilities
use std::thread;
use std::sync::mpsc;

use rand::Rng;

// import VRC client behavior
use crate::client::Client;
use crate::traits::{Input, Avatar};

fn main() {
    cli::init_logger("trace");

    info!("Started!");

    let vrc = Client::new(9001, 9000);

    // create communication between threads
    let (tx, rx) = mpsc::channel::<u8>();

    vrc.input_test();

    let mut rng = rand::thread_rng();

    thread::spawn(move || {

        loop {
            vrc.chatbox_message(format!(
                r#"rico's bs
            currently in development 🙏
            testing multithreading...
            received {} from main thread"#, rx.recv().unwrap()).as_str()
                );
            cli::sleep(5000);
        }
        
    });

    loop {
        tx.send(rng.gen()).unwrap();
        cli::sleep(4999);
    }
}
