// library import
extern crate rosc;

use crate::cli;

//rosc encoder
use rosc::encoder;
// rosc types
use rosc::{OscMessage, OscPacket, OscType};

use log::{debug, info, warn, error};
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

trait SendData { 
    fn send_data(&self, param_name: &str, param_arg: Vec<OscType>);
}

#[derive(Debug)]
pub struct Client {
    currently_playing: String,
    address: SocketAddrV4,
    sock: UdpSocket
}

impl SendData for Client {
    fn send_data(&self, param_name: &str, param_arg: Vec<OscType>) {
        // create OSC/1.0 Message buffer with parameter name and parameter value/arg
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: param_name.to_string(),
            args: param_arg,
        }))
        .unwrap();

        // sends the encoded Message buffer to VRChat on port 9000
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

    // tests if game socket connection is open
    pub fn test_socket(&self) -> bool {
        todo!();
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

    // jump takes ints 1 and 0 -> 1 is activated 0 is reset
    pub fn input_jump(&self) {
        let param_name = format!("/input/Jump");
        self.send_data(&param_name, vec![OscType::Int(1)]); // activate jump
        cli::sleep(10); // required sleep time for "keypresses" to register
        self.send_data(&param_name, vec![OscType::Int(0)]) // reset jump
    }

    // run takes ints 1 and 0 -> 1 is activated 0 is inactive
    pub fn input_run(&self, toggle: i32) {
        let param_name = format!("/input/Run");
        let param_arg: OscType = OscType::Int(toggle);
        self.send_data(&param_name, vec![param_arg]) // 1 = running | 0 = walking
    }

    pub fn chatbox_message(&self, message: &str) { 
        // IMPLEMENT THIS AT COMPILE TIME LATER
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

    pub fn input_init(&self) {
        self.send_data("/input/Jump", vec![OscType::Int(0)]); // init jump to 0
        cli::sleep(10);
        self.send_data("/input/Run", vec![OscType::Int(0)]); // init run to 0

        debug!("Initialized jump and run inputs")
    }

    // this is all super hardcoded, its just a demonstration and its kinda cool in public lobbies
    pub fn test_actions(&self) {
        // moving left for 1750ms = 360 degrees

        // ensure that you can run and jump before moving
        self.input_init();

        self.chatbox_message("calibrating movement..."); // no it does not calibrate movement lol

        cli::sleep(500);

        self.chatbox_message("calibrating movement -> Left");
        // 360 degrees left
        self.input_look("Left", true);
        self.input_move("Forward", true);
        cli::sleep(1775);
        self.input_move("Forward", false);
        self.input_look("Left", false);

        self.chatbox_message("calibrating movement -> Right");
        // 360 degrees right
        self.input_look("Right", true);
        self.input_move("Backward", true);
        cli::sleep(1775);
        self.input_move("Backward", false);
        self.input_look("Right", false);

        self.chatbox_message("calibrating movement -> Forward");
        self.input_run(1);
        self.input_move("Forward", true);
        cli::sleep(1000);
        self.input_move("Forward", false);
        self.input_run(0);

        self.chatbox_message("calibrating movement -> Backward");
        self.input_run(1);
        self.input_move("Backward", true);
        cli::sleep(1000);
        self.input_move("Backward", false);
        self.input_run(0);

        cli::sleep(100);
        self.chatbox_message("calibrating movement -> Jumping");
        self.input_jump();
        cli::sleep(500);
        self.input_jump();
        cli::sleep(500);
        self.input_jump();

        self.chatbox_message("bless up 🙏");

        cli::sleep(3000);
    }
}