// process info
use sysinfo::{Pid, System};

// import logging features
use log::{debug, warn, info};

pub struct Proc {
    pub pid: Option<sysinfo::Pid>,
    pub window_name: Option<String>,
    pub desired_name: String,
}

impl Proc {
    // pass in process name
    pub fn new(desired_proc_name: &str) -> Self {
        let system = System::new_all();

        let (pid, window_name) = system
            .processes()
            .iter()
            .find(|(_, process)| process.name() == desired_proc_name)
            .map(|(pid, process)| (Some(*pid), Some(process.name().to_string())))
            .unwrap_or((None, None));

        if let Some(pid) = pid {
            info!("Found process '{}' with PID {}", desired_proc_name, pid);
        } 
        else {
            warn!("Process '{}' not found", desired_proc_name);
        }

        Proc {
            pid,
            window_name,
            desired_name: desired_proc_name.to_string(),
        }
    }


    pub fn update(&mut self, ) {
        let system = System::new_all();

        let (pid, window_name) = system
            .processes()
            .iter()
            .find(|(_, process)| process.name() == self.desired_name)
            .map(|(pid, process)| (Some(*pid), Some(process.name().to_string())))
            .unwrap_or((None, None));

        if let Some(pid) = pid {
            info!("Found process '{}' with PID {}", self.desired_name, pid);
        } 
        else {
            warn!("Process '{}' not found", self.desired_name);
        }

        self.pid = pid;
        self.window_name = window_name;
    }
}