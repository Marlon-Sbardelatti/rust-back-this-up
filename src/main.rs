use std::fs;
use std::path::PathBuf;
mod function;
use clap::{Arg, Command};
use function::function::{backup, copy_dir, get_current_dir};

fn main() {
    let params = Command::new("back-this-up")
        .version("1.0.0")
        .about("A simple cli tool to quickly backup files")
        .author("Marlon Sbardelatti, @Marlon-Sbardelatti")
        .arg(
            Arg::new("git")
                .short('g')
                .long("git")
                .help("Push the backup dir to git"),
        )
        .arg(
            Arg::new("backup")
                .short('b')
                .long("backup")
                .help("backup the files"),
        )
        .arg(
            Arg::new("back-git")
                .short('a')
                .long("back-git")
                .help("backup the files and push to git"),
        )
        .get_matches();

    let home_path = dirs::home_dir();
    if let Some(path) = home_path {
        let back_this_up_path = path.join("back-this-up");
        if back_this_up_path.exists() {
            let user_path: PathBuf = get_current_dir();
            if user_path.exists() {
                let mut found = false;
                if let Some(_param) = params.get_one::<String>("git") {
                    // copy_dir(&user_path, &back_this_up_path);
                    backup();
                    found = true;
                }
                if let Some(_param) = params.get_one::<String>("backup") {
                    copy_dir(&user_path, &back_this_up_path);
                    found = true;
                }
                if let Some(_param) = params.get_one::<String>("back-git") {
                    copy_dir(&user_path, &back_this_up_path);
                    backup();
                    found = true;
                }

                if !found {
                   println!("You have to provide a flag");
                }
            } else {
                println!("User path doesnt exists");
            }
        } else {
            match fs::create_dir(back_this_up_path) {
                Ok(_) => {
                    println!("Created back_this_up_path");
                    println!("Run the program again");
                }
                Err(e) => println!("{}", e),
            }
        }
    }
}
