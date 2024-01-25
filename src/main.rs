mod client;
mod cli;

use log::{debug, info, warn, error};
use crate::client::Client;

fn main() {
    cli::init_logger("trace");

    info!("Started!");

    let vrc = Client::new();

    vrc.input_move("Forward", true);

    cli::sleep(1000);

    vrc.input_move("Forward", false);

    loop {
        vrc.chatbox_message("rico's bs -> TESTING OSC SOCKET CONNECTION (let me know if you see this please)");

        cli::sleep(20000);
    }
}
