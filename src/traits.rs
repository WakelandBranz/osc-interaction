// required for OSC functionality
use rosc::OscType;

pub trait SendData { 
    fn send_data(&self, param_name: &str, param_arg: Vec<OscType>);
}

// input traits for client
pub trait Input {
    /* 
     * AXES
     */

    // forward and backward movement, more precise than input_move
    // vertical takes f32 from -1 to 1
    fn input_vertical(&self, direction: f32);

    // left and right movement, more precise than input_move
    // horizontal takes f32 from -1 to 1
    fn input_horizontal(&self, velocity: f32);

    /*
     * BUTTONS
     */
    // directions are 'Forward', 'Backward', 'Left', 'Right'
    fn input_move(&self, direction: &str, toggle: bool);

    // directions are 'Left' and 'Right'
    fn input_look(&self, direction: &str, toggle: bool);

    // jump takes i32 1 and 0 -> 1 is activated 0 is reset
    fn input_jump(&self);

    // run takes i32 1 and 0 -> 1 is activated 0 is inactive
    fn input_run(&self, toggle: i32);

    // takes inputs s b n
    // s = chatbox text  | can be sent as a raw string
    // b = don't open keyboard (post straight to chatbox)
    // n = don't play notification sound
    fn chatbox_message(&self, message: &str);
}

// output traits for client (receiving data from surroundings)
pub trait Avatar {

}