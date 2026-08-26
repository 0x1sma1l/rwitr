use std::path::PathBuf;

use clap::{Args, Parser};

/// why is xxxx running?
#[derive(Parser, Debug)]
#[command(version, about, override_usage = "rwitr [TARGETS...] [OPTIONS]")]
struct Cli {
    #[command(flatten)]
    target: TargetArgs,

    #[command(flatten)]
    options: OptionsArgs,
}

#[derive(Args, Debug)]
struct TargetArgs {
    /// Process name to inspect.
    name: Option<String>,

    /// Look up a process by PID.
    #[arg(short = 'p', long)]
    pid: Option<u32>,

    /// Find the process listening on a port.
    #[arg(short = 'o', long)]
    port: Option<u16>,

    /// Find the process holding a file open.
    #[arg(short = 'f', long)]
    file: Option<PathBuf>,

    /// Inspect a container by name.
    #[arg(short = 'c', long)]
    container: Option<String>,
}

#[derive(Args, Debug)]
struct OptionsArgs {
    /// Match the process name exactly.
    #[arg(short = 'e', long)]
    exact: bool,

    /// Show the full process ancestry.
    #[arg(short = 't', long)]
    tree: bool,

    /// Show only suspicious or noteworthy warnings.
    #[arg(short = 'w', long)]
    warnings: bool,
}

impl Cli {
    fn into_request(self) -> Request {
        // translate into application model

        unimplemented!();
    }
}

struct Request {
    targets: Vec<Target>,
    options: Options,
}

enum Target {
    Name(String),
    Pid(u32),
    Port(u16),
    File(PathBuf),
    Container(String),
}

struct Options {
    exact: bool,
    tree: bool,
    warnings: bool,
}

fn main() {
    let _cli = Cli::parse();
}
