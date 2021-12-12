use std::path;
use std::process;

pub const VEXCODE: &str = "C:/Program Files (x86)/Vex Robotics/VEXcode Pro V5";
pub const TOOLCHAIN: &str= "/toolchain/vexv5/win32";

pub fn build(file: &std::path::PathBuf) -> std::result::Result<(), ()> {
    let vexcode = path::PathBuf::from(VEXCODE);

    if !vexcode.exists() {
        eprintln!("VexCode V5 pro is not installed! Please install it to use vexer");
        Err(())
    } else {
        let toolchain = format!("{}{}", VEXCODE, TOOLCHAIN);
        let make = [ &toolchain, "/tools/bin/make.exe"].concat();
        let sdk = [ &VEXCODE, "/sdk" ].concat();

        let args = [
            &["T=", &sdk.as_str()].concat(),
            "V=1"
        ];

        println!("\"{}\" {}", make.as_str(), args.join(" "));
        
        let output = if cfg!(target_os = "windows") {
            process::Command::new(make.as_str())
                    .args(args)
                    .current_dir(std::fs::canonicalize(file).expect("Failed to turn file into path!"))
                    .env("path", [
                        std::env::var("path").expect("Failed to fetch PATH env variable!"),
                        [ toolchain.as_str(), "/clang/bin" ].concat(),
                        [ toolchain.as_str(), "/gcc/bin" ].concat(),
                        [ toolchain.as_str(), "/tools/bin" ].concat()
                    ].join(";"))
                    .output()
                    .expect("failed to execute process")
        } else {
            process::Command::new("sh")
                    .arg("-c")
                    .arg("echo hello")
                    .output()
                    .expect("failed to execute process")
        };

        use std::io::Write;

        std::io::stdout().write_all(&output.stdout).unwrap();
        std::io::stderr().write_all(&output.stderr).unwrap();

        Ok(())
    }
}