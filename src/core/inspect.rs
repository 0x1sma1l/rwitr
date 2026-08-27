use std::path::PathBuf;

use crate::core::model::{Request, Target};

pub fn inspect(request: Request) {
    for target in request.targets {
        match target {
            Target::Name(name) => {
                inspect_name(name);
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

fn inspect_name(name: String) {
    // this handles the Target::Name arm
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
