use std::path::PathBuf;

#[derive(Debug)]
pub struct Request {
    pub targets: Vec<Target>,
    pub options: Options,
}

#[derive(Debug)]
pub enum Target {
    Name(String),
    Pid(u32),
    Port(u16),
    File(PathBuf),
    Container(String),
}

#[derive(Debug)]
pub struct Options {
    pub exact: bool,
    pub tree: bool,
    pub warnings: bool,
}
