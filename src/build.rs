use std::path;
use std::process;

pub const VEXCODE: &str = "C:/Program Files (x86)/Vex Robotics/VEXcode Pro V5";
pub const TOOLCHAIN: &str = "/toolchain/vexv5/win32";

pub fn build(file: &std::path::PathBuf) -> std::result::Result<(), ()> {
    let vexcode = path::PathBuf::from(VEXCODE);

    if !vexcode.exists() {
        eprintln!("VexCode V5 pro is not installed! Please install it to use vexer");
        Err(())
    } else {
        let toolchain = [VEXCODE, TOOLCHAIN].concat();
        let make = [&toolchain, "/tools/bin/make.exe"].concat();
        let sdk = [&VEXCODE, "/sdk"].concat();

        let args = [&["T=", &sdk.as_str()].concat(), "V=0"];

        let path = if cfg!(target_os = "windows") {
            "path"
        } else {
            "PATH"
        };

        std::fs::remove_dir_all(file.join("build")).expect("Failed to delete old build!");

        let mut child = 
            process::Command::new(make.as_str())
                .args(args)
                .current_dir(file)
                .env(
                    path,
                    [
                        std::env::var(path).expect("Failed to fetch PATH env variable!"),
                        [toolchain.as_str(), "/clang/bin"].concat(),
                        [toolchain.as_str(), "/gcc/bin"].concat(),
                        [toolchain.as_str(), "/tools/bin"].concat(),
                    ]
                    .join(";"),
                )
                .spawn()
                .expect("failed to execute process");

        let ecode = child.wait()
            .expect("failed to wait on child");

        if !ecode.success() {
            println!("Build Failed!");
            return Err(())
        }

        Ok(())
    }
}
