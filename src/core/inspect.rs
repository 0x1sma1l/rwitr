use std::{ffi::OsStr, path::PathBuf};
use sysinfo::{Pid, Process, ProcessRefreshKind, ProcessesToUpdate, System};

use crate::core::model::{ProcessInfo, Request, Target};

pub fn inspect(request: Request) {
    let mut sys = System::new();

    for target in request.targets {
        match target {
            Target::Name(name) => {
                inspect_name(&mut sys, name);
            }
            Target::Pid(p) => {
                println!("{p}");
            }
            Target::Port(p) => {
                println!("{p}");
            }
            Target::File(path) => {
                println!("{:?}", path);
            }
            Target::Container(c) => {
                println!("{c}");
            }
        }
    }
}

fn inspect_name(sys: &mut System, name: String) {
    let name = OsStr::new(&name);

    sys.refresh_processes_specifics(ProcessesToUpdate::All, true, ProcessRefreshKind::nothing());
    let pids: Vec<Pid> = sys.processes_by_name(name).map(|p| p.pid()).collect();

    sys.refresh_processes(ProcessesToUpdate::Some(&pids), true);
    let processes: Vec<&Process> = pids
        .into_iter()
        .filter_map(|pid| sys.process(pid))
        .collect();

    println!("{:#?}", processes);
}

fn inspect_pid(pid: u32) {
    todo!();
}

fn inspect_port(port: u16) {
    todo!();
}

fn inspect_file(path: PathBuf) {
    todo!();
}

fn inspect_container(container: String) {
    todo!();
}

fn convert_process(process: &Process) -> ProcessInfo {
    ProcessInfo {
        pid: process.pid().as_u32(),
        parent: process.parent().map(|p| p.as_u32()),
        name: process.name().to_string_lossy().to_string(),
        executable: process.exe().map(|path| path.to_path_buf()),
        cwd: process.cwd().map(|path| path.to_path_buf()),
        status: process.status().into(),
        user_id: process.user_id().map(|uid| uid.to_be()),
    }
}
