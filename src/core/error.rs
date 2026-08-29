#[derive(Debug)]
pub enum AppError {
    ProcessNotFound(u32),
    ProcessNameNotFound(String),
    PortNotFound(u16),
}
