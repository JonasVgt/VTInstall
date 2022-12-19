mod errors;

use std::{fs::{self, File}, path::Path, io::Write};

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
    let target_path = root.join(script.name().to_owned() + ".sh");

    if target_path.exists() {
        return Err(CompileError::script_duplicate_name_error(format!("File at: {} already exists!", target_path.to_str().unwrap_or(""))));
    }

    let mut file = match File::create(target_path){
        Ok(f) => f,
        Err(error) => return Err(CompileError::io_error(error.to_string()))
    };

    if let Err(error) = file.write( "#!/bin/bash\n".as_bytes()){
        return Err(CompileError::io_error(error.to_string()));
    }



    for statement in script.statements() {
        if let Err(error) = file.write( statement.to_bash(dry_run).as_bytes()){
            return Err(CompileError::io_error(error.to_string()));
        }

        if let Err(error) = file.write( "\n".as_bytes()){
            return Err(CompileError::io_error(error.to_string()));
        }
    }
    Ok(())
}
