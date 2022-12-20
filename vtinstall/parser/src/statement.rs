use crate::script::instruction::Instruction;




#[derive(Debug, PartialEq)]
pub struct Statement {
    instruction: Instruction,
    args: Vec<String>,
}

impl Statement {
    pub fn new(instruction: Instruction, args: Vec<String>) -> Self {
        Self { instruction, args }
    }

    pub fn get_executable(&self, dry_run: bool) -> &str{
        self.instruction.get_executable(dry_run)
    }


    pub fn args(&self) -> &[String] {
        self.args.as_ref()
    }
}

