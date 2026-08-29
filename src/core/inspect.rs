use std::{ffi::OsStr, path::PathBuf};
use sysinfo::{Pid, Process, ProcessRefreshKind, ProcessesToUpdate, System, Users};

use crate::core::{
    error::AppError,
    model::{Options, ProcessInfo, Request, Target},
};

pub fn inspect(request: Request) -> Result<Vec<ProcessInfo>, AppError> {
    let mut sys = System::new();
    let users = Users::new_with_refreshed_list();
    let options = request.options;

    let mut processes = Vec::new();

    for target in request.targets {
        match target {
            Target::Name(name) => {
                let result = inspect_name(&mut sys, &users, &options, name)?;
                processes.extend(result);
            }
            Target::Pid(pid) => {
                let result = inspect_pid(&mut sys, &users, pid)?;
                processes.push(result);
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

    Ok(processes)
}

fn inspect_name(
    sys: &mut System,
    users: &Users,
    options: &Options,
    name: String,
) -> Result<Vec<ProcessInfo>, AppError> {
    let os_name = OsStr::new(&name);

    sys.refresh_processes_specifics(ProcessesToUpdate::All, true, ProcessRefreshKind::nothing());

    let pids: Vec<Pid> = if !options.exact {
        sys.processes_by_name(os_name).map(|p| p.pid()).collect()
    } else {
        sys.processes_by_exact_name(os_name)
            .map(|p| p.pid())
            .collect()
    };

    sys.refresh_processes(ProcessesToUpdate::Some(&pids), true);
    let raw_processes: Vec<&Process> = pids
        .into_iter()
        .filter_map(|pid| sys.process(pid))
        .collect();

    let processes: Vec<ProcessInfo> = raw_processes
        .into_iter()
        .map(|process| convert_process(process, users))
        .collect();

    if processes.is_empty() {
        Err(AppError::ProcessNameNotFound(name))
    } else {
        Ok(processes)
    }
}

fn inspect_pid(sys: &mut System, users: &Users, pid: u32) -> Result<ProcessInfo, AppError> {
    sys.refresh_processes_specifics(ProcessesToUpdate::All, true, ProcessRefreshKind::nothing());

    sys.refresh_processes(ProcessesToUpdate::Some(&[Pid::from_u32(pid)]), true);

    match sys.process(Pid::from_u32(pid)) {
        Some(process) => Ok(convert_process(process, users)),
        None => Err(AppError::ProcessNotFound(pid)),
    }
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

fn convert_process(process: &Process, users: &Users) -> ProcessInfo {
    let username = process
        .user_id()
        .and_then(|uid| users.get_user_by_id(uid))
        .map(|user| user.name().to_string());

    ProcessInfo {
        pid: process.pid().as_u32(),
        parent: process.parent().map(|p| p.as_u32()),
        name: process.name().to_string_lossy().to_string(),
        executable: process.exe().map(|path| path.to_path_buf()),
        cwd: process.cwd().map(|path| path.to_path_buf()),
        status: process.status().into(),
        user_name: username,
    }
}
