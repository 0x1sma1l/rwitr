use std::path::{Path, PathBuf};

use sysinfo::ProcessStatus::{
    Dead, Idle, LockBlocked, Parked, Run, Sleep, Stop, Suspended, Tracing,
    UninterruptibleDiskSleep, Unknown, Wakekill, Waking, Zombie,
};

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

#[derive(Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub parent: Option<u32>,
    pub name: String,
    pub executable: Option<PathBuf>,
    pub cwd: Option<PathBuf>,
    pub status: ProcessStatus,
    pub user_name: Option<String>,
}

#[derive(Debug)]
pub enum ProcessStatus {
    Idle,
    Run,
    Sleep,
    Stop,
    Zombie,
    Unknown,
}

impl From<sysinfo::ProcessStatus> for ProcessStatus {
    fn from(value: sysinfo::ProcessStatus) -> Self {
        match value {
            Idle => Self::Idle,
            Run => Self::Run,
            Sleep => Self::Sleep,
            Stop => Self::Stop,
            Zombie => Self::Zombie,
            Tracing => Self::Unknown,
            Dead => Self::Unknown,
            Wakekill => Self::Unknown,
            Waking => Self::Unknown,
            Parked => Self::Unknown,
            LockBlocked => Self::Unknown,
            UninterruptibleDiskSleep => Self::Unknown,
            Suspended => Self::Unknown,
            Unknown(_) => Self::Unknown,
        }
    }
}
