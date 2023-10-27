pub mod function {
    extern crate fs_extra;
    use dirs;
    use fs_extra::dir::{self, CopyOptions};
    use std::env;
    use std::process::Command;
    use std::{fs, path::PathBuf};
    pub fn copy_dir(source: &PathBuf, destination: &PathBuf) {
        let options = CopyOptions::new();

        if let Err(e) = dir::copy(source, destination, &options) {
            println!("Error: {:?}", e);
        } else {
            println!("Directory copied successfully.");
        }
    }

    pub fn get_current_dir() -> PathBuf {
        let current = env::current_dir();
        let mut path = PathBuf::new();
        match current {
            Ok(p) => {
                path = p;
            }
            Err(e) => println!("{}", e),
        }
        path
    }

    pub fn backup() {
        let command = r#"
        git add .
        git commit -m "test"
        git push -u origin master
        "#;

        let output = Command::new("bash")
            .current_dir("/home/hetzwga/back-this-up")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            println!("Command executed successfully!");
            let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 in stdout");
            println!("Output: {}", stdout);
        } else {
            eprintln!("Command failed with error code: {:?}", output.status);
            let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8 in stderr");
            eprintln!("Error: {}", stderr);
        }
    }
}
