mod errors;

use std::{fs, os, path::Path};

use parser::script::Script;

use self::errors::CompileError;

pub fn compile(script: Script, target: &Path, dry_run: bool) -> Result<(), CompileError> {
    if target.exists() {
        return Err(CompileError::target_exists_error(format!("Target path already exists ({})!", target.to_str().unwrap_or(""))));
    }

    if let Err(error) = fs::create_dir_all(&target) {
        return Err(CompileError::io_error(error.to_string()));
    }

    if let Err(error) = compile_script(script, &target, dry_run){
        return Err(error);
    }

    Ok(())
}

fn compile_script(script: Script, root: &Path, dry_run: bool) -> Result<(), CompileError> {
    let target = root.join(script.name().to_owned() + ".sh");

    if target.exists() {
        return Err(CompileError::script_duplicate_name_error(format!("File at: {} already exists!", target.to_str().unwrap_or(""))));
    }

    if let Err(error) = fs::write(&target, "#!/bin/bash"){
        return Err(CompileError::io_error(error.to_string()));
    }

    for statement in script.statements() {
        if let Err(error) = fs::write(&target, statement.to_bash(dry_run)){
            return Err(CompileError::io_error(error.to_string()));
        }
    }
    Ok(())
}
