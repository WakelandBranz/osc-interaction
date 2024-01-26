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
use std::sync::{Arc, Mutex};

// import VRC client behavior
use crate::client::Client;
use crate::traits::{Input, Avatar, RxTx};

fn main() {
    cli::init_logger("trace");

    info!("Started!");

    let vrc_client: Arc<Client> = Arc::new(Client::new(9001, 9000));

    // clone the Arc for each thread that needs access to the vrc client
    let vrc_thread1 = Arc::clone(&vrc_client);

    vrc_client.input_test();

    thread::spawn(move || {
        loop {
            vrc_thread1.chatbox_message(format!(
                r#"rico's bs
            currently in development 🙏
            testing multithreading..."#).as_str());
            cli::sleep(3000);
        }
        
    });

    info!("Preparing to log all incoming OSC packets at {}", vrc_client.get_rx_addr());
    cli::sleep(3000);
    loop {
        vrc_client.recv_data();
    }
}
