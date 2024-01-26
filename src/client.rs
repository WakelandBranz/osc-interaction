// library import
extern crate rosc;

// import from cli.rs
use crate::cli;
// rosc encoder
use rosc::encoder;
// rosc types
use rosc::{OscMessage, OscPacket, OscType};
// import from traits.rs
use crate::traits::{RxTx, Input, Avatar};

use log::{debug, info, warn, error};
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use std::sync::{mpsc, Mutex, Arc};

#[derive(Debug)]
pub struct Client {
    currently_playing: String,
    rx_addr: SocketAddrV4, // receiver
    rx_addr_str: String,
    tx_addr: SocketAddrV4, // transmitter
    tx_addr_str: String,
    sock: UdpSocket
}

impl RxTx for Client {
    fn send_data(&self, param_name: &str, param_arg: Vec<OscType>) {
        // create OSC/1.0 Message buffer with parameter name and parameter value/arg
        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: param_name.to_string(),
            args: param_arg,
        }))
        .unwrap();

        // sends the encoded Message buffer to VRChat on port 9000
        // send to requires a String as its address:port
        // allows for multithreading
        self.sock.send_to(&msg_buf, self.tx_addr_str.clone()).unwrap();
    }

    fn recv_data(&self) {

        // create/allocate buffer on the stack with a size of MTU
        let mut buf = [0u8; rosc::decoder::MTU];
        
        // continuously read OSC data from port 9001.
        loop {
            /*
                receive OSC data length in var "buffer_len". Address of origin data in "a".
                writes the data received to the buffer on the stack "buf".
            */
            let (buffer_len, _a) = self.sock.recv_from(&mut buf).unwrap();
            /*
                checks that the packet is greater than 0 bytes.
                if the packet length is <= 0 then the recv loop is restarted.
                the received buffer is then decoded and parsed.
                if the decoded packet "pkt" is of OSC type Message
                the OSC address and OSC args are printed to the CLI.
            */
            if buffer_len <= 0 {
            } 
            else {
                let pkt = match rosc::decoder::decode_udp(&buf) {
                    Ok(pkt) => pkt,
                    Err(_e) => {
                        error!("{}", "!!! Invalid OSC buffer !!!");
                        panic!("Failed to read OSC buffer")
                    },
                };
                match pkt.1 {
                    OscPacket::Message(msg) => {
                        debug!("OSC Address: {}", msg.addr);
                        debug!("OSC Arguments: {:?}", msg.args);
                        break;
                    },
                    _ => {}
                }
            }
        }
        
        
    }
}

impl Input for Client {
    /* 
     * AXES
     */

    // forward and backward movement, more precise than input_move
    // vertical takes f32 from -1 to 1
    fn input_vertical(&self, velocity: f32) {
        let param_name: String = "/input/Vertical".to_string();
        let param_arg: OscType = OscType::Float(velocity);
        self.send_data(&param_name, vec![param_arg])
    }

    // left and right movement, more precise than input_move
    // horizontal takes f32 from -1 to 1
    fn input_horizontal(&self, velocity: f32) {
        let param_name: String = "/input/Horizontal".to_string();
        let param_arg: OscType = OscType::Float(velocity);
        self.send_data(&param_name, vec![param_arg])
    }

    // forward and backward movement for a held object
    // takes f32 from -1 to 1
    fn input_move_hold(&self, velocity: f32) {
        let param_name: String = "/input/MoveHoldFB".to_string();
        let param_arg: OscType = OscType::Float(velocity);
        self.send_data(&param_name, vec![param_arg])
    }

    // clockwise and counter-clockwise movement for a held object
    // takes f32 from -1 to 1
    fn input_spin_hold_cw(&self, velocity: f32) {
        let param_name: String = "/input/SpinHoldCwCcw".to_string();
        let param_arg: OscType = OscType::Float(velocity);
        self.send_data(&param_name, vec![param_arg])
    }

    // up and down movement for a held object
    // takes f32 from -1 to 1
    fn input_spin_hold_vertical(&self, velocity: f32) {
        let param_name: String = "/input/SpinHoldUD".to_string();
        let param_arg: OscType = OscType::Float(velocity);
        self.send_data(&param_name, vec![param_arg])
    }

    // left and right movement for a held object
    // takes f32 from -1 to 1
    fn input_spin_hold_horizontal(&self, velocity: f32) {
        let param_name: String = "/input/SpinHoldLR".to_string();
        let param_arg: OscType = OscType::Float(velocity);
        self.send_data(&param_name, vec![param_arg])
    }

    /*
     * BUTTONS
     */

    // directions are 'Forward', 'Backward', 'Left', 'Right'
    fn input_move(&self, direction: &str, toggle: bool) {
        let param_name: String = format!("/input/Move{}", direction);
        let param_arg: OscType = OscType::Bool(toggle);
        self.send_data(&param_name, vec![param_arg])
    }

    // directions are 'Left' and 'Right'
    fn input_look(&self, direction: &str, toggle: bool) {
        let param_name: String = format!("/input/Look{}", direction);
        let param_arg: OscType = OscType::Bool(toggle);
        self.send_data(&param_name, vec![param_arg])
    }

    // jump takes ints 1 and 0 -> 1 is activated 0 is reset
    fn input_jump(&self) {
        let param_name: String = "/input/Jump".to_string();
        self.send_data(&param_name, vec![OscType::Int(1)]); // activate jump
        cli::sleep(10); // required sleep time for "keypresses" to register
        self.send_data(&param_name, vec![OscType::Int(0)]) // reset jump
    }

    // run takes ints 1 and 0 -> 1 is activated 0 is inactive
    fn input_run(&self, toggle: i32) {
        let param_name: String = "/input/Run".to_string();
        let param_arg: OscType = OscType::Int(toggle);
        self.send_data(&param_name, vec![param_arg]) // 1 = running | 0 = walking
    }

    // takes inputs s b n
    // s = chatbox text | can be sent as a raw string
    // b = don't open keyboard (post straight to chatbox)
    // n = don't play notification sound
    fn chatbox_message(&self, message: &str) { 
        // IMPLEMENT THIS AT COMPILE TIME LATER
        // truncate message to max length (144) if it's over that length
        let mut verified_message: &str = &message;
        if message.len() > 144 {
            verified_message = cli::truncate(message, 144);
            warn!("Message length > 144! Automatically truncated to length 144.")
        }

        // error checking for future reference
        let time: String = cli::string_system_time();
        debug!("Sent '{}' at {}", &verified_message, &time);

        let param_name: &str = "/chatbox/input"; // destination
        // args
        let param_arg: Vec<OscType> = vec![
            OscType::String(format!("{} | {}", time, verified_message)), // chatbox text
            OscType::Bool(true), // don't open keyboard (post straight to chatbox)
            OscType::Bool(false)]; // don't play notification sound
        self.send_data(param_name, param_arg)
    }
}

impl Client {
    // requires two ports to bind to, first is the receive port, second is the query port
    pub fn new(rx_port: u16, tx_port: u16) -> Self {  

        // always listen to port 9001 by default
        let rx_addr: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), rx_port);
        // always query to port 9000 by default
        let tx_addr: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), tx_port);

        // ensure that socket has bound successfully
        let socket = match UdpSocket::bind(&rx_addr) {
            // if successful, print to debug and assign to variable
            Ok(success) =>  {
                debug!("Sucessfully bound to {:?}", &rx_addr);
                success
            }
            // if unsuccessful, print to error and panic close program
            Err(e) =>  {
                error!("Failed to bind to {:?}, is your VRChat client open?", &rx_addr);
                panic!("Error: {:?}", e);
            }
        };

        debug!("Binding to {} | Info will be sent to {}", &rx_addr, &tx_addr);

        Client {
            currently_playing: String::new(),
            rx_addr,
            tx_addr,
            rx_addr_str: format!("{}:{}", rx_addr.ip(), rx_addr.port()),
            tx_addr_str: format!("{}:{}", tx_addr.ip(), tx_addr.port()),
            sock: socket
        }
    }

    // tests if game socket connection is open
    pub fn test_socket(&self) -> bool {
        todo!();
    }

    // ensure that you can run and jump before moving, i'm not sure how necessary this really is
    pub fn input_button_init(&self) {
        self.send_data("/input/Jump", vec![OscType::Int(0)]); // init jump to 0
        cli::sleep(10);
        self.send_data("/input/Run", vec![OscType::Int(0)]); // init run to 0

        debug!("Initialized jump and run inputs")
    }

    // this is all super hardcoded, its just a demonstration and its kinda cool in public lobbies
    pub fn input_test(&self) {
        // moving left for 1750ms = 360 degrees

        debug!("!!! Check OSC debug menu to ensure that these actions are functional !!!");

        self.input_button_init(); // ensure that you can run and jump before moving

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
        self.input_vertical(0.5);
        cli::sleep(1000);
        self.input_vertical(0.0);
        self.input_run(0);

        self.chatbox_message("calibrating movement -> Backward");
        self.input_run(1);
        self.input_vertical(-0.5);
        cli::sleep(1000);
        self.input_vertical(0.0);
        self.input_run(0);

        cli::sleep(100);
        self.chatbox_message("calibrating movement -> Jumping");
        self.input_jump();
        cli::sleep(500);
        self.input_jump();
        cli::sleep(500);
        self.input_jump();

        // spacing in timing as to not exceed ratelimit
        self.input_move_hold(1.0);
        cli::sleep(200);
        self.input_move_hold(-1.0);

        self.input_spin_hold_cw(1.0);
        cli::sleep(200);
        self.input_spin_hold_cw(-1.0);

        self.input_spin_hold_vertical(1.0);
        cli::sleep(200);
        self.input_spin_hold_vertical(-1.0);

        self.input_spin_hold_horizontal(1.0);
        cli::sleep(200);
        self.input_spin_hold_horizontal(-1.0);

        self.chatbox_message("bless up 🙏");

        cli::sleep(3000);
    }
}