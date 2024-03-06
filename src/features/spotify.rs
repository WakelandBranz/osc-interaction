// import proc functionality
use crate::features::proc::Proc;

// import time functionality 
use std::time::{Instant, Duration};

use log::debug;

pub struct Song {
    name: String,
    artist: String,
}

pub struct Spotify {
    pub song: Song,
    pub session_start: Instant
}

impl Spotify {
    pub fn new() -> Self {

        let spotify_proc = Proc::new("Spotify");

        debug!("Spotify process: Pid -> {:?} | Window name -> {:?}", spotify_proc.pid, spotify_proc.window_name);

        let test_song = Song {
            name: "poop".to_string(),
            artist: "fart".to_string()
        };
        
        Spotify {
            song: test_song,
            session_start: Instant::now()
        }
    }

    pub fn update(&mut self) {
        let spotify_proc = Proc::new("Spotify");

        debug!("UPDATING Spotify process: Pid -> {:?} | Window name -> {:?}", spotify_proc.pid, spotify_proc.window_name);

        let test_song = Song {
            name: "poop".to_string(),
            artist: "fart".to_string()
        };

        self.song = test_song;
    } 

    fn get_session_length(&self) -> Duration {
        self.session_start.elapsed()
    }
}