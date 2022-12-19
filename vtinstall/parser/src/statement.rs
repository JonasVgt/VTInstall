use crate::instruction::Instruction;

#[derive(Debug, PartialEq)]
pub struct Statement{
    instruction: Instruction,
    args: Vec<String>    
}

impl Statement {
    pub fn new(instruction: Instruction, args: Vec<String>) -> Self { Self { instruction, args } }

    pub fn execute(&self, dry_run: bool){
        self.instruction.execute(dry_run, &self.args);
    }

    pub fn to_bash(&self, dry_run: bool) -> &str{
        self.instruction.to_bash(dry_run)
    }

}