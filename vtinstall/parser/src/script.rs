use crate::statement::Statement;

#[derive(Debug, PartialEq)]
pub struct Script {
    name: String,
    statements: Vec<Statement>,
}

impl Script {
    /// Creates a new [`Script`].
    pub fn new(statements: Vec<Statement>, name :String) -> Self {
        Self { statements, name}
    }

    /// Returns a reference to the statements of this [`Script`].
    pub fn statements(&self) -> &[Statement] {
        self.statements.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
