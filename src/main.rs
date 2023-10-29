use std::fs;
use std::path::PathBuf;
mod git;
use clap::{Arg, Command};
mod drive;
use drive::functions::{drive_backup, drive_copy_dir, get_drive_current_dir};
use git::functions::{get_git_current_dir, git_backup, git_copy_dir};

fn main() {
    let params = Command::new("back-this-up")
        .version("1.0.0")
        .about("A simple cli tool to quickly git_backup files")
        .author("Marlon Sbardelatti, @Marlon-Sbardelatti")
        .arg(
            Arg::new("git")
                .short('g')
                .long("gp")
                .help("Push the back-this-up dir to git"),
        )
        .arg(
            Arg::new("git_backup")
                .short('b')
                .long("gb")
                .help("backup the files in back-this-up dir"),
        )
        .arg(
            Arg::new("back-git")
                .short('a')
                .long("gbp")
                .help("Backup the files and push to git"),
        )
        .arg(
            Arg::new("back-push-drive")
                .short('i')
                .long("dbp")
                .help("Backup the files and push to google drive"),
        )
        .arg(
            Arg::new("back-drive")
                .short('s')
                .long("db")
                .help("Backup the files in drive-back-this-up dir"),
        )
        .arg(
            Arg::new("drive")
                .short('d')
                .long("dp")
                .help("Push the drive-back-this-up dir to google drive"),
        )
        .get_matches();

    let mut found: [bool; 2] = [false, false];

    let home_path = dirs::home_dir();
    if let Some(path) = home_path {
        let git_back_this_up_path = path.join("back-this-up");
        if git_back_this_up_path.exists() {
            let user_path: PathBuf = get_git_current_dir();
            if user_path.exists() {
                if let Some(_param) = params.get_one::<String>("git") {
                    git_backup();
                    found[0] = true;
                }
                if let Some(_param) = params.get_one::<String>("git_backup") {
                    git_copy_dir(&user_path, &git_back_this_up_path);
                    found[0] = true;
                }
                if let Some(_param) = params.get_one::<String>("back-git") {
                    git_copy_dir(&user_path, &git_back_this_up_path);
                    git_backup();
                    found[0] = true;
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
                if let Some(_param) = params.get_one::<String>("back-drive") {
                    drive_copy_dir(&user_path, &drive_back_this_up_path);
                    found[1] = true;
                }
                if let Some(_param) = params.get_one::<String>("back-push-drive") {
                    drive_copy_dir(&user_path, &drive_back_this_up_path);
                    drive_backup();
                    found[1] = true;
                }
                if let Some(_param) = params.get_one::<String>("drive") {
                    drive_backup();
                    found[1] = true;
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
        if !found[1] && !found[0] {
            println!("You have to provide a flag");
        }
    }
}
