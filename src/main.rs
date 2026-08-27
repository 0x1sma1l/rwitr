use std::path::PathBuf;
mod core;

use crate::core::inspect::inspect;
use crate::core::model::{Options, Request, Target};

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
    fn into_request(self) -> Result<Request, String> {
        let mut targets = Vec::new();

        if let Some(value) = self.target.name {
            targets.push(Target::Name(value));
        }

        if let Some(value) = self.target.pid {
            targets.push(Target::Pid(value));
        }

        if let Some(value) = self.target.port {
            targets.push(Target::Port(value));
        }

        if let Some(value) = self.target.file {
            targets.push(Target::File(value));
        }

        if let Some(value) = self.target.container {
            targets.push(Target::Container(value));
        }

        if targets.is_empty() {
            // no valid target was provided, therefore handle error here...
            return Err(String::from("No valid target was provided"));
        }

        let req = Request {
            targets,
            options: Options {
                exact: self.options.exact,
                tree: self.options.tree,
                warnings: self.options.warnings,
            },
        };

        Ok(req)
    }
}

fn main() {
    let cli = Cli::parse();
    let request = cli.into_request().unwrap();

    inspect(request);
}
