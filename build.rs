use std::env;
use std::io::Result;
use std::path::Path;
use std::process::{Command, ExitStatus};

fn run_cmd_in_dir<'a, I>(working_dir: &Path, prog_name: &str, args: I) -> Result<ExitStatus>
where
    I: Iterator<Item = &'a str>,
{
    Command::new("npm")
        .args(args)
        .current_dir(working_dir)
        .status()
}

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let a = &["install"];
    run_cmd_in_dir(&Path::new(&project_dir), "npm", &["install"]).unwrap();
}
