use crate::statement::Statement;

use super::Script;

pub struct ScriptBuilder {
    name: String,
    statements: Vec<Statement>,
}

impl ScriptBuilder {
    pub fn new() -> ScriptBuilder {
        ScriptBuilder {
            name: String::new(),
            statements: Vec::new(),
        }
    }

    pub fn add_statement(mut self, statement: Statement) -> ScriptBuilder {
        self.statements.push(statement);
        self
    }

    pub fn statements(mut self, statements: Vec<Statement>) -> ScriptBuilder {
        self.statements = statements;
        self
    }

    pub fn build(self) -> Script {
        Script {
            name: self.name,
            statements: self.statements,
        }
    }

    pub fn name(mut self, name: String) -> ScriptBuilder {
        self.name = name;
        self
    }
}
