pub trait Rule {
    fn new() -> Self;
}

pub trait Packet {
    fn new() -> Self;
}

pub trait Connection {
    fn new() -> Self;
}

pub struct ServerBuilder {
    connections: Vec<Box<Connection>>
}

impl ServerBuilder {
    fn new() -> Self {
        ServerBuilder {
            connections: Vec::new()
        }
    }
    fn add<C: Connection>(self, connection: C) -> Self {
        self.connections.push(Box::new(connection));
        ServerBuilder {
            connections: self.connections
        }
    }
}

pub struct Server {
}

pub struct IntegrationBuilder<R: Rule, P: Packet> {
    rule: Option<R>,
    packet: Option<P>,
    Server_builder: ServerBuilder
}

impl<R: Rule, P: Packet> IntegrationBuilder<R,P> {
    pub fn new() -> Self {
        IntegrationBuilder{
            rule: None,
            packet: None,
            server_builder: ServerBuilder::new()
        }
    }

    pub fn rule(self, rule: R) -> Self {
        IntegrationBuilder{
            rule: rule,
            packet: self.packet,
            server_builder: self.server_builder
        }
    }

    pub fn packet(self, packet: P) -> Self {
        IntegrationBuilder{
            rule: self.rule,
            packet: packet,
            server_builder: self.server_builder
        }
    }

    pub fn connection<C: Connection>(self, connection: C) -> Self {
        let server_builder = self.server_builder.add(connection);
        IntegrationBuilder{
            rule: self.rule,
            packet: self.packet,
            server_builder: server_builder
        }
    }

    pub fn build(self) -> Integration<R,P> {
        //let server = server_builder.build();
        Integration{
            rule: self.rule,
            packet: self.packet,
            //server: server
        }
    }
}

pub struct Integration<R: Rule, P: Packet> {
    rule: R,
    packet: P,
    //server: Server
}
