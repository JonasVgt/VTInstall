use std::{
    fmt::Debug,
    io::Write,
    process::{Command, Stdio},
};

use regex::internal::Inst;

pub struct Instruction {
    name: String,
    executable: String,
    dry_run_executable: String,
}

impl Instruction {
    pub fn get_run_instruction() -> Instruction {
        Instruction {
            name: String::from("RUN"),
            executable: String::from("$*;"),
            dry_run_executable: String::from("echo hello;"),
        }
    }

    pub fn execute(&self, dry_run: bool, args: &Vec<String>) {
        let exec = if dry_run {
            &self.dry_run_executable
        } else {
            &self.executable
        };

        let mut cmd = Command::new(exec).args(args).spawn().unwrap();
        cmd.wait().unwrap();
    }

    pub fn to_bash(&self, dry_run: bool) -> &str {
        if dry_run {
            self.dry_run_executable.as_str()
        } else {
            self.executable.as_str()
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
