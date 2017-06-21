extern crate rusty_wolf_common;

use rusty_wolf_common::Packet;

pub struct SimplePacket {
}

impl Packet for SimplePacket {
    fn new() -> Self {
        SimplePacket{}
    }
}
