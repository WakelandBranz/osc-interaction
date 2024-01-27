// import VRC client input and output
mod client;
// import traits for VRC client
mod traits;
// import CLI functions
mod cli;
// import config functions
mod config;

// import logging features
use log::{debug, info, warn, error};

use std::net::UdpSocket;
use std::sync::atomic::AtomicU16;
// import threading capabilities
use std::thread;
use std::sync::Arc;

// import VRC client behavior
use crate::client::Client;
use crate::traits::{Input, Avatar, Data};
use crate::config::Config;

fn main() {
    cli::init_logger("info");

    info!("Started!");

    let config = Config::new();

    let vrc_client: Arc<Client> = Arc::new(Client::new(9001, 9000));

    // clone the Arc for each thread that needs access to the vrc client
    let vrc_thread1 = Arc::clone(&vrc_client);
    
    thread::spawn(move || {
        vrc_thread1.input_test();
        loop {
            vrc_thread1.chatbox_message(format!(
            r#"wakeland's projects
            currently in development 🙏
            testing multithreading..."#).as_str());
            cli::sleep(3000);
        }
        
    });

    info!("Preparing to log all incoming OSC packets at {}", vrc_client.rx_addr);
    cli::sleep(3000);
    loop {
        vrc_client.recv_data();
    }
}
