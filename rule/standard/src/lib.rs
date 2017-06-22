extern crate rusty_wolf_common;

use rusty_wolf_common::Rule;

pub struct StandardRule {
}

impl Rule for StandardRule {}

impl StandardRule {
    pub fn new() -> Self {
        StandardRule{}
    }
}

pub struct StandardRuleBuilder {
}

impl StandardRuleBuilder {
    pub fn new() -> Self {
        StandardRuleBuilder{}
    }
    pub fn set_agent_num(&self, agent_num: u32) -> Self {
        StandardRuleBuilder{}
    }
    pub fn build(&self) -> StandardRule {
        StandardRule{}
    }
}
