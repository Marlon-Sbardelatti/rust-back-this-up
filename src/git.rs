pub mod functions {
    extern crate fs_extra;
    use fs_extra::dir::{self, CopyOptions};
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;
    pub fn git_copy_dir(source: &PathBuf, destination: &PathBuf) {
        let options = CopyOptions::new();

        let y = &source.to_str().unwrap();
        let x = y.to_string();
        let git_path = PathBuf::from(x + "/.git");

        let mut has_git = false;
        if git_path.exists() {
            has_git = true;
        }

        let mut deleted = false;
        if let Err(_e) = dir::copy(source, destination, &options) {
            let y = &destination.to_str().unwrap();
            let x = y.to_string();
            let dir_name = source.file_name().unwrap().to_string_lossy().to_string();
            let dir_ref = &dir_name;
            let path = PathBuf::from(x + "/" + dir_ref + "/");
            match fs::remove_dir_all(path) {
                Ok(_) => deleted = true,
                Err(e) => println!("{}", e),
            }
        } else {
            println!("Directory copied successfully.");
        }

        if deleted {
            if let Err(e) = dir::copy(source, destination, &options) {
                println!("Error: {:?}", e);
            } else {
                println!("Directory with the same name was replaced successfully.");
            }
        }

        if has_git {
            let y = &destination.to_str().unwrap();
            let x = y.to_string();
            let dir_name = source.file_name().unwrap().to_string_lossy().to_string();
            let dir_ref = &dir_name;
            let git_path = PathBuf::from(x + "/" + dir_ref + "/");
            let command = r#"
                rm -r -f -v .git
            "#;
            let output = Command::new("bash")
                .current_dir(git_path)
                .arg("-c")
                .arg(command)
                .output()
                .expect("Failed to execute command");
            if output.status.success() {
                println!("Command executed successfully!");
            } else {
                eprintln!("Command failed with error code: {:?}", output.status);
                let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8 in stderr");
                eprintln!("Error: {}", stderr);
            }
        }
    }

    pub fn get_git_current_dir() -> PathBuf {
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

    pub fn git_backup() {
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
