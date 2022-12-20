use std::fmt::Debug;

pub struct Instruction {
    name: String,
    executable: String,
    args: Vec<String>,
}

impl Instruction {
    pub fn get_run_instruction(args: Vec<String>) -> Instruction {
        Instruction {
            name: String::from("RUN"),
            executable: String::from("run.sh"),
            args
        }
    }

    pub fn executable(&self) -> &str {
        self.executable.as_ref()
    }

    pub fn args(&self) -> &[String] {
        self.args.as_ref()
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instruction")
            .field("name", &self.name)
            .finish()
    }
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
