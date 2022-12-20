use std::fmt::Debug;

pub struct Instruction {
    name: String,
    executable: String,
    dry_run_executable: String,
}

impl Instruction {
    pub fn get_run_instruction() -> Instruction {
        Instruction {
            name: String::from("RUN"),
            executable: String::from("run.sh"),
            dry_run_executable: String::from("run_dr.sh"),
        }
    }

    pub fn get_executable(&self, dry_run: bool) -> &str {
        if dry_run {
            &self.dry_run_executable
        } else {
            &self.executable
        }
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
