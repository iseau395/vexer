use structopt::StructOpt;

mod device;

mod build;

/// A structure for the arguments
#[derive(std::fmt::Debug, StructOpt)]
#[structopt(about = "Build VEXCode V5 Pro projects and download them to the V5 brain")]
struct Vexer {
    /// The command to run
    command: String,

    /// The path to the input file
    #[structopt(short, long, default_value = ".", parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Vexer::from_args();

    match args.command.as_str() {
        "build" => {
            build::build(&args.path).expect("Build Failed");
        }
        _ => eprintln!("Unknown command, \"{}\"", args.command)
    }
}