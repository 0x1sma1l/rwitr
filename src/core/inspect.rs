use listeners::Protocol;
use std::{
    ffi::OsStr,
    io::ErrorKind,
    path::{Path, PathBuf},
    process::Command,
};
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
            Target::Port(port) => {
                let result = inspect_port(&mut sys, &users, port)?;
                processes.extend(result);
            }
            Target::File(path) => {
                let result = inspect_file(&mut sys, &users, path)?;
                processes.extend(result);
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

fn inspect_port(sys: &mut System, users: &Users, port: u16) -> Result<Vec<ProcessInfo>, AppError> {
    let mut pids = Vec::new();

    if let Ok(p) = listeners::get_process_by_port(port, Protocol::TCP) {
        pids.push(Pid::from_u32(p.pid));
    }

    if let Ok(p) = listeners::get_process_by_port(port, Protocol::UDP) {
        pids.push(Pid::from_u32(p.pid));
    }

    sys.refresh_processes_specifics(ProcessesToUpdate::All, true, ProcessRefreshKind::nothing());
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
        Err(AppError::PortNotFound(port))
    } else {
        Ok(processes)
    }
}

fn inspect_file(
    sys: &mut System,
    users: &Users,
    path: PathBuf,
) -> Result<Vec<ProcessInfo>, AppError> {
    if !path.exists() {
        return Err(AppError::FileNotFound(path));
    }

    let pids: Vec<Pid> = get_pids_via_lsof(&path)?
        .into_iter()
        .map(Pid::from_u32)
        .collect();

    sys.refresh_processes_specifics(ProcessesToUpdate::All, true, ProcessRefreshKind::nothing());
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
        Err(AppError::NoProcessUsingFile(path))
    } else {
        Ok(processes)
    }
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

fn get_pids_via_lsof(file_path: &Path) -> Result<Vec<u32>, AppError> {
    let output = Command::new("lsof").arg("-t").arg(file_path).output();

    match output {
        Ok(out) if out.status.success() => {
            let stdout_str = String::from_utf8_lossy(&out.stdout);
            let pids = stdout_str
                .lines()
                .filter_map(|line| line.trim().parse::<u32>().ok())
                .collect();

            Ok(pids)
        }
        Err(error) if error.kind() == ErrorKind::NotFound => Err(AppError::LsofUnavailable),

        _ => {
            // Some other IO error.
            todo!()
        }
    }
}
