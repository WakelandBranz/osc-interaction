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
        vrc.chatbox_message("rico's bs -> osc socket connection test lmk if you can see this when it updates (gonna make this into a spotify updater later) 🙏");
        cli::sleep(5000);
    }
}
