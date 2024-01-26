// import VRC client input and output
mod client;
// import traits for VRC client
mod traits;
// import CLI functions
mod cli;

// import logging features
use log::{debug, info, warn, error};

// import VRC client behavior
use crate::client::Client;
use crate::traits::{Input, Avatar};

fn main() {
    cli::init_logger("trace");

    info!("Started!");

    let vrc = Client::new(9001, 9000);

    vrc.input_test();

    loop {
        vrc.chatbox_message(
            r#"rico's bs
            currently in development 🙏"#);
        cli::sleep(5000);
    }
}
