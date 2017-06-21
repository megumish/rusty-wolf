extern crate rusty_wolf_common;

use rusty_wolf_common::Connection;

pub struct SimpleStdConnection {
}

impl Connection for SimpleStdConnection {
    fn new() -> Self {
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
}
