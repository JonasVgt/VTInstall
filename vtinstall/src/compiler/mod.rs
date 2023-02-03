mod errors;
mod script_writer;

use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use parser::script::{command, statement::Statement, Script};

use self::{errors::CompileError, script_writer::ScriptWriter};

pub fn compile(source: &Path, target: &Path, dry_run: bool) -> Result<(), CompileError> {
    let input = fs::read_to_string(source).unwrap();

    let script = parser::parse(
        input.as_str(),
        source
            .file_stem()
            .map_or("script", |x| x.to_str().unwrap_or("script")),
    )
    .unwrap();

    if target.exists() {
        return Err(CompileError::target_exists_error(format!(
            "Target path already exists ({})!",
            target.to_str().unwrap_or("")
        )));
    }

    if let Err(error) = fs::create_dir_all(&target) {
        return Err(CompileError::io_error(error.to_string()));
    }

    if let Err(error) = compile_script(script, &source.parent().unwrap_or(Path::new("./")), &target)
    {
        return Err(error);
    }

    Ok(())
}

fn compile_script(
    script: Script,
    source_root: &Path,
    target_root: &Path,
) -> Result<(), CompileError> {
    let target_path = target_root.join(script.name().to_owned() + ".sh");

    let mut writer = ScriptWriter::create(target_path.as_path())?;


    for statement in script.statements() {
        match statement {
            Statement::UNKNOWN => {}
            Statement::USE(_) => {}
            Statement::COMMAND(instruction) => {
                let executable = instruction.executable();

                let target_instruction = target_root.join("cmd").join(&executable);
                let source_instruction = source_root.join("cmd").join(&executable);
                if !&target_instruction.exists() {
                    if !&source_instruction.exists() {
                        return Err(CompileError::instruction_not_found_error(format!(
                            "Instruction at: {} not found!",
                            source_instruction.to_str().unwrap_or("")
                        )));
                    }

                    if !&target_root.join("cmd").exists() {
                        if let Err(error) = fs::create_dir(&target_root.join("cmd")) {
                            return Err(CompileError::io_error(error.to_string()));
                        }
                    }

                    if let Err(error) = fs::copy(&source_instruction, &target_instruction) {
                        return Err(CompileError::io_error(format!(
                            "from: {} to: {}: {}",
                            &source_instruction.to_str().unwrap_or(""),
                            &target_instruction.to_str().unwrap_or(""),
                            error.to_string()
                        )));
                    }
                }
                writer.command(executable, instruction.args())?;

            }
        }
    }
    Ok(())
}
