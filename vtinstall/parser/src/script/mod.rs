pub mod builder;
pub mod instruction;
pub mod statement;

use self::{builder::ScriptBuilder, statement::Statement};

#[derive(Debug, PartialEq)]
pub struct Script {
    name: String,
    statements: Vec<Statement>,
}

impl Script {

    pub fn builder() -> ScriptBuilder{
        ScriptBuilder::new()
    }

    /// Returns a reference to the statements of this [`Script`].
    pub fn statements(&self) -> &[Statement] {
        self.statements.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
