use std::env;
use std::io::Result;
use std::path::Path;
use std::process::{Command, ExitStatus};

fn run_cmd_in_dir(working_dir: &Path, prog_name: &str, args: &[&str]) -> Result<ExitStatus>
{
  Command::new(prog_name)
    .args(args)
    .current_dir(working_dir)
    .status()
}

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    match run_cmd_in_dir(&Path::new(&project_dir), "npm", &["install"]) {
      Ok(_) => println!("Done"),
      Err(err) => println!("received error: {}", err)
    }
}
