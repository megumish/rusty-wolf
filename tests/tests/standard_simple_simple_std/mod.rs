use rusty_wolf_rule_standard::*;
use rusty_wolf_packet_simple::*;
use rusty_wolf_connection_simple_std::*;
use rusty_wolf_common::*;

#[test]
fn it_works() {
    let integration_builder = IntegrationBuilder::new();

    let rule = StandardRuleBuilder::new()
                                   .set_agent_num(15)
                                   .build();
    let integration_builder = integration_builder.rule(rule);

    let packet = SimplePacket::new();
    let integration_builder = integration_builder.packet(packet);
    
    let process = ProcessBuilder::new("hoge.elf")
                                 .args(vec!["-hoge piyo","-nyan nyuru"])
                                 .set_agent_num(15) // or skipping this function, the connection is executed as padding at the end.
                                 .build();
    let connection = SimpleStdConnection::new_with_process(process);
    let integration = integration_builder.connection(connection)
                                         .build();

    integration.run();
}
