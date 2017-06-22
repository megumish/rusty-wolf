pub trait Rule{}

pub trait Packet{}

pub trait Connection{}

pub struct ServerBuilder<'wolf>{
    connections: Vec<Box<Connection + 'wolf>>
}

impl<'wolf> ServerBuilder<'wolf> {
    fn new() -> Self {
        ServerBuilder {
            connections: Vec::new()
        }
    }
    fn add<C: Connection + 'wolf>(mut self, connection: C) -> Self {
        self.connections.push(Box::new(connection));
        ServerBuilder {
            connections: self.connections
        }
    }
}

pub struct Server {
}

pub struct IntegrationBuilder<'wolf, R: Rule, P: Packet> {
    rule: Option<R>,
    packet: Option<P>,
    server_builder: ServerBuilder<'wolf>
}

impl<'wolf, R: Rule, P: Packet> IntegrationBuilder<'wolf, R, P> {
    pub fn new() -> Self {
        IntegrationBuilder{
            rule: None,
            packet: None,
            server_builder: ServerBuilder::new()
        }
    }

    pub fn rule(self, rule: R) -> Self {
        IntegrationBuilder{
            rule: Some(rule),
            packet: self.packet,
            server_builder: self.server_builder
        }
    }

    pub fn packet(self, packet: P) -> Self {
        IntegrationBuilder{
            rule: self.rule,
            packet: Some(packet),
            server_builder: self.server_builder
        }
    }

    pub fn connection<C: Connection + 'wolf>(self, connection: C) -> Self {
        let server_builder = self.server_builder.add(connection);
        IntegrationBuilder{
            rule: self.rule,
            packet: self.packet,
            server_builder: server_builder
        }
    }

    pub fn build(self) -> Integration<R, P> {
        //let server = server_builder.build();
        Integration{
            rule: self.rule.unwrap(),
            packet: self.packet.unwrap(),
            //server: server
        }
    }
}

pub struct Integration<R: Rule, P: Packet> {
    rule: R,
    packet: P,
    //server: Server
}

impl<R: Rule, P: Packet> Integration<R, P> {
    pub fn run(self) {}
}
