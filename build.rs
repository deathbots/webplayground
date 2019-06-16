use std::env;
use std::io::Result;
use std::path::Path;
use std::process::{Command, ExitStatus};
use std::ffi::OsStr;
use std::fmt::Debug;

fn run_cmd_in_dir<I, S>(working_dir: &Path, prog_name: &str, args: I) -> Result<ExitStatus>
where
    I: IntoIterator<Item=S> + Debug, S: AsRef<OsStr>,
{
    println!("Running {} with arguments {:?} in {}", prog_name, args, working_dir.display());
    Command::new(prog_name)
        .args(args)
        .current_dir(working_dir)
        .status()
}

fn main() {
    println!("Starting build script");
    let project_dir_raw = env::var("CARGO_MANIFEST_DIR").unwrap();
    let ui_dir = &Path::new(&project_dir_raw).join("ui");
    println!("Executing NPM Install");
    run_cmd_in_dir(ui_dir, "npm", &["install"]).unwrap();
    let profile = env::var("PROFILE").unwrap();
    let mut webpack_target = "development";
    let p: &str = profile.as_ref();
    if p == "release" {
        webpack_target = "production";
    }
    println!("Executing webpack");
    run_cmd_in_dir(ui_dir, "npx", &["webpack", "--mode", webpack_target]).unwrap();

}

