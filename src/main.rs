use std::fs;
use std::path::PathBuf;
mod function;
use clap::{Arg, Command};
use function::function::{
    drive_backup, drive_copy_dir, get_drive_current_dir, get_git_current_dir, git_backup,
    git_copy_dir,
};

fn main() {
    let params = Command::new("back-this-up")
        .version("1.0.0")
        .about("A simple cli tool to quickly git_backup files")
        .author("Marlon Sbardelatti, @Marlon-Sbardelatti")
        .arg(
            Arg::new("git")
                .short('g')
                .long("git")
                .help("Push the back-this-up dir to git"),
        )
        .arg(
            Arg::new("git_backup")
                .short('b')
                .long("git_backup")
                .help("backup the files in back-this-up dir"),
        )
        .arg(
            Arg::new("back-git")
                .short('a')
                .long("back-git")
                .help("Backup the files and push to git"),
        )
        .arg(
            Arg::new("back-push-drive")
                .short('i')
                .long("bp-drive")
                .help("Backup the files and push to google drive"),
        )
        .arg(
            Arg::new("back-drive")
                .short('s')
                .long("back-drive")
                .help("Backup the files in drive-back-this-up dir"),
        )
        .arg(
            Arg::new("drive")
                .short('d')
                .long("drive")
                .help("Push the drive-back-this-up dir to google drive"),
        )
        .get_matches();

    let home_path = dirs::home_dir();
    if let Some(path) = home_path {
        let git_back_this_up_path = path.join("back-this-up");
        if git_back_this_up_path.exists() {
            let user_path: PathBuf = get_git_current_dir();
            if user_path.exists() {
                let mut found = false;
                if let Some(_param) = params.get_one::<String>("git") {
                    // git_copy_dir(&user_path, &git_back_this_up_path);
                    git_backup();
                    found = true;
                }
                if let Some(_param) = params.get_one::<String>("git_backup") {
                    git_copy_dir(&user_path, &git_back_this_up_path);
                    found = true;
                }
                if let Some(_param) = params.get_one::<String>("back-git") {
                    git_copy_dir(&user_path, &git_back_this_up_path);
                    git_backup();
                    found = true;
                }

                if !found {
                    println!("You have to provide a flag");
                }
            } else {
                println!("User path doesnt exists");
            }
        } else {
            match fs::create_dir(git_back_this_up_path) {
                Ok(_) => {
                    println!("Created git-back-this-up folder");
                    println!("Run the program again");
                }
                Err(e) => println!("{}", e),
            }
        }
        let drive_back_this_up_path = path.join("drive-back-this-up");
        if drive_back_this_up_path.exists() {
            let user_path: PathBuf = get_drive_current_dir();
            if user_path.exists() {
                let mut found = false;
                if let Some(_param) = params.get_one::<String>("back-drive") {
                    drive_copy_dir(&user_path, &drive_back_this_up_path);
                    found = true;
                }
                if let Some(_param) = params.get_one::<String>("back-push-drive") {
                    drive_copy_dir(&user_path, &drive_back_this_up_path);
                    drive_backup();
                    found = true;
                }
                if let Some(_param) = params.get_one::<String>("drive") {
                    drive_backup();
                    found = true;
                }
                if !found {
                    println!("You have to provide a flag");
                }
            } else {
                println!("User path doesnt exists");
            }
        } else {
            match fs::create_dir(drive_back_this_up_path) {
                Ok(_) => {
                    println!("Created drive-back-this-up folder");
                }
                Err(e) => println!("{}", e),
            }
        }
    }
}
