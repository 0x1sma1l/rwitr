use std::path::PathBuf;

use clap::Parser;

/// why is this running? 
#[derive(Parser, Debug)]
#[command(version, about, long_about= None)]
struct Cli {
   name: Option<String>,

   #[arg(short = 'p', long)]
   pid: Option<u32>,

   #[arg(short = 'o', long)]
   port: Option<u16>,

   #[arg(short = 'f', long)]
   file: Option<PathBuf>,

   #[arg(short = 'c', long)]
   container: Option<String>,

   #[arg(short = 'e', long)]
   exact: bool,

   #[arg(short = 't', long)]
   tree: bool,

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
    println!("Hello, world!");
}
