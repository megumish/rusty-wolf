extern crate rusty_wolf_common;

use rusty_wolf_common::Connection;

pub struct SimpleStdConnection {
}

impl Connection for SimpleStdConnection {}

impl SimpleStdConnection {
    pub fn new() -> Self {
        SimpleStdConnection{}
    }
    
    pub fn new_with_process(process: Process) -> Self {
        SimpleStdConnection{}
    }
}

pub struct Process {
}

pub struct ProcessBuilder {
}

impl ProcessBuilder {
    pub fn new<S: Into<String>>(exec_name: S) -> Self {
        ProcessBuilder{}
    }

    pub fn args<S: Into<String>>(self, arg_vec: Vec<S>) -> Self {
        ProcessBuilder{}
    }
    
    pub fn set_agent_num(self, agent_num: u32) -> Self {
        ProcessBuilder{}
    }

    pub fn build(self) -> Process {
        Process{}
    }
}
