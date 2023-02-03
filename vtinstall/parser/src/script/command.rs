use std::fmt::Debug;

pub struct Command {
    name: String,
    executable: String,
    args: String,
}

impl Command {
    pub fn get_run_instruction(args: &str) -> Command {
        Command {
            name: String::from("RUN"),
            executable: String::from("run.sh"),
            args: String::from(args)
        }
    }

    pub fn executable(&self) -> &str {
        self.executable.as_ref()
    }

    pub fn args(&self) -> &str {
        self.args.as_ref()
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instruction")
            .field("name", &self.name)
            .finish()
    }
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
