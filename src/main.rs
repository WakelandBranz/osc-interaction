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
use std::sync::{Arc, Mutex};

// import VRC client behavior
use crate::client::Client;
use crate::traits::{Input, Avatar, Data};
use crate::config::Config;

fn main() {
    cli::init_logger("info");

    info!("Started!");

    // parse config info
    // this info can be reparsed during runtime
    let config = Arc::new(Mutex::new(Config::new()));
    let vrc_client: Arc<Client> = Arc::new(Client::new(9001, 9000));

    // clone the Arc for each thread that needs access to certain variables
    let vrc_thread1 = Arc::clone(&vrc_client);
    let config_thread1 = Arc::clone(&config);

    thread::spawn(move || {
        //vrc_thread1.input_test();
        loop {
            // acquire the lock on the config
            let mut config_guard = config_thread1.lock().unwrap();

            // update the config
            *config_guard = Config::new();

            let features = config_guard.features.as_ref().unwrap();
            let message_text = features.text.as_deref().unwrap_or_default();

            vrc_thread1.chatbox_message(format!("{} | {}", cli::string_system_time(), message_text).as_str()); // figure out config.message usage later


            vrc_thread1.input_rotate_axis_left();
            cli::sleep(1000);

            vrc_thread1.input_rotate_axis_right();
            cli::sleep(1000);
        }
    });

    info!("Logging all incoming OSC packets at {}", vrc_client.rx_addr);
    loop {
        //vrc_client.recv_data();
        cli::sleep(100);
    }
}
