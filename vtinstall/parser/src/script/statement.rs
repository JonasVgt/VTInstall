
use crate::script::command::Command;

use super::use_statement::UseStatement;

#[derive(Debug, PartialEq)]
pub enum Statement {
    COMMAND(Command),
    USE(UseStatement),
    UNKNOWN
}

impl Statement {
    pub fn parse(instruction: String, args: String) -> Self{
        match instruction.as_str() {
            "RUN" => Self::COMMAND(Command::get_run_instruction(&args)),
            "USE" => Self::USE(UseStatement::parse(&args).unwrap()),
            _ => Self::UNKNOWN
        }
    }
}
