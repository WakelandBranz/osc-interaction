// library import
extern crate rosc;

use crate::cli;

//rosc encoder
use rosc::{encoder, OscArray};
// rosc types
use rosc::{OscMessage, OscPacket, OscType};

use log::{debug, info, warn, error};
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

trait send_data { 
    fn send_data (&self, param_name: &str, param_arg: Vec<OscType>);
}

#[derive(Debug)]
pub struct Client {
    currently_playing: String,
    address: SocketAddrV4,
    sock: UdpSocket
}

impl send_data for Client {

    fn send_data(&self, param_name: &str, param_arg: Vec<OscType>) {
        // Create OSC/1.0 Message buffer with parameter name and parameter value/arg
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: param_name.to_string(),
            args: param_arg,
        }))
        .unwrap();

        // Sends the encoded Message buffer to VRChat on port 9000
        self.sock.send_to(&msg_buf, self.address).unwrap();
    }
}

impl Client {
    pub fn new() -> Self {
        debug!("Binding to 127.0.0.1:9001 | Info will be sent to 127.0.0.1:9000");

        Client {
            currently_playing: String::new(),
            address: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0 , 1), 9000),
            sock: UdpSocket::bind(format!("127.0.0.1:9001")).unwrap()
        }
    }

    // directions are 'Forward', 'Backward', 'Left', 'Right'
    pub fn input_move(&self, direction: &str, toggle: bool) {
        let param_name = format!("/input/Move{}", direction);
        let param_arg: OscType = OscType::Bool(toggle);
        self.send_data(&param_name, vec![param_arg])
    }

    // directions are 'Left' and 'Right'
    pub fn input_look(&self, direction: &str, toggle: bool) {
        let param_name = format!("/input/Look{}", direction);
        let param_arg: OscType = OscType::Bool(toggle);
        self.send_data(&param_name, vec![param_arg])
    }

    pub fn input_Jump(&self) {
        let param_name = format!("/input/Jump");
        let param_arg: OscType = OscType::Bool(false); // placeholder variable FIX LATER !!!!!!
        self.send_data(&param_name, vec![param_arg])
    }

    pub fn chatbox_message(&self, message: &str) { 
        // implement this at compile time later
        // truncate message to max length (144) if it's over that length
        let mut verified_message: &str = &message;
        if message.len() > 144 {
            verified_message = cli::truncate(message, 144);
            warn!("Message length > 144! Automatically truncated to length 144.")
        }

        // error checking for future reference
        let time = cli::string_system_time();
        debug!("Sent '{}' at {}", &verified_message, &time);

        let param_name = "/chatbox/input"; // destination
        // args
        let param_arg: Vec<OscType> = vec![
            OscType::String(format!("{} | {}", time, verified_message)), // chatbox text
            OscType::Bool(true), // don't open keyboard (post straight to chatbox)
            OscType::Bool(false)]; // don't play notification sound
        self.send_data(param_name, param_arg)
    }
}