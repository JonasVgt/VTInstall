
use crate::script::command::Command;


#[derive(Debug, PartialEq)]
pub enum Statement {
    COMMAND(Command),
}

impl Statement {




    pub fn parse(instruction: String, args: String) -> Self{
        return Self::COMMAND(Command::get_run_instruction(args));
    }
}



