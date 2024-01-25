mod client;
mod cli;

use log::{debug, info, warn, error};
use crate::client::Client;

fn main() {
    cli::init_logger("trace");

    info!("Started!");

    let vrc = Client::new();

    vrc.test_actions();

    loop {
        vrc.chatbox_message("rico's bs -> testing message connection, sorry if messages are annoying 🙏");

        cli::sleep(20000);
    }
}
