use structopt::StructOpt;

mod build;

#[cfg(target_os = "windows")]
pub const HOME: &str = "userprofile";

#[cfg(target_os = "linux")]
pub const HOME: &str = "HOME";

pub const VEXCODE_DIR: &str = "/.vexer/vexcode";

/// A structure for the arguments
#[derive(StructOpt)]
#[derive(std::fmt::Debug)]
struct Cli {
    /// The command to run
    command: String,

    /// The path to the input file
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[allow(unused_variables)]

fn main() {
    let args = Cli::from_args();

    if args.command == String::from("build") {
        build::build(&args.path).expect("Build Failed");
    }

    println!("{:?}, {:?}", args.command, args.path);
}