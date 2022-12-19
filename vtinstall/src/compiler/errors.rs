use std::fmt::{Display, Debug, write};




pub enum ErrorKind{
    TargetExistsError,
    IOError,
    ScriptDuplicateNameError
}


impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::IOError => write!(f, "IO ERROR"),
            ErrorKind::ScriptDuplicateNameError => write!(f, "SCRIPT DUPLICATE NAME ERROR"),
            ErrorKind::TargetExistsError => write!(f, "TARGET EXISTS ERROR")
        }
    }
}

pub struct CompileError{
    kind: ErrorKind,
    message: String
}


impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl CompileError {
    
    pub fn io_error(message: String) -> CompileError{
        CompileError{
            kind: ErrorKind::IOError,
            message
        }
    }

    pub fn script_duplicate_name_error(message: String) -> CompileError{
        CompileError{
            kind: ErrorKind::ScriptDuplicateNameError,
            message
        }
    }
    pub fn target_exists_error(message: String) -> CompileError{
        CompileError{
            kind: ErrorKind::TargetExistsError,
            message
        }
    }
}