use rusty_wolf_rule::standard::RuleBuilder as RuleBuilder;
use rusty_wolf_packet::simple::Packet as Packet;
use rusty_wolf_connection::simple_std::Connection as Connection;
use rusty_wolf_connection::simple_std::ProcessBuilder as WolfProcessBuilder;
use rusty_wolf_common::Integration as Integration;

#[test]
fn it_works() {
    let rule = RuleBuilder::new()
                           .set_agent_num(15)
                           .build();

    let packet = Packet::new();
    
    let process = WolfProcessBuilder::new("hoge.elf")
                                     .args(vec!["-hoge piyo","-nyan nyuru"]
                                     .set_agent_num(15) // or skipping this function, the connection is executed as padding at the end.
                                     .build();
    let connection = Connection::new_with_process(process)

    let integration = Integration::new(rule,packet,connection)
    integration.run();
}
