use std::{fs::File, io::Write, path::Path};

use super::errors::CompileError;

pub struct ScriptWriter {
    file: File,
}

impl ScriptWriter {
    pub fn create(path: &Path) -> Result<ScriptWriter, CompileError> {
        if path.exists() {
            return Err(CompileError::target_exists_error(format!(
                "File at: {} already exists!",
                path.to_str().unwrap_or("UNKNOWN")
            )));
        }

        let mut file = match File::create(path) {
            Ok(f) => f,
            Err(error) => return Err(CompileError::io_error(error.to_string())),
        };

        if let Err(error) = file.write("#!/bin/bash\nBASEDIR=$(dirname \"$0\")\n".as_bytes()) {
            return Err(CompileError::io_error(error.to_string()));
        }

        Ok(ScriptWriter { file })
    }

    pub fn command(&mut self, executable: &str, args: &str) -> Result<(), CompileError> {
        if let Err(error) = self
            .file
            .write(format!("\"$BASEDIR/cmd/{}\" {}\n", executable, args).as_bytes())
        {
            return Err(CompileError::io_error(error.to_string()));
        }
        Ok(())
    }
}
