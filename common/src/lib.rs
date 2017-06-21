pub trait Rule {
}

pub trait Packet {
}

pub trait Connection {
}

pub struct Integration<R: Rule,P: Packet,C: Connection> {
    rule: R,
    packet: P,
    connection: C
}

pub struct IntegrationBuilder<R: Rule,
