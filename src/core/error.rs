use std::path::PathBuf;

#[derive(Debug)]
pub enum AppError {
    ProcessNotFound(u32),
    ProcessNameNotFound(String),
    PortNotFound(u16),
    FileNotFound(PathBuf),
    NoProcessUsingFile(PathBuf),
    LsofUnavailable,
}
