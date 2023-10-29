# back-this-up

**Version**: 1.0.0

A simple command-line tool to facilitate the backup and management of files. Designed for easy backup to both Git and Google Drive.

**Author**: Marlon Sbardelatti

## Overview

`back-this-up` is a command-line tool that simplifies the process of backing up files to both Git and Google Drive. It provides options to perform the following tasks:

- **Git Operations:**
  - Push the contents of the `back-this-up` directory to Git.
  - Backup the files in the `back-this-up` directory.
  - Backup the files and push to Git.

- **Google Drive Operations:**
  - Copy the contents of the `drive-back-this-up` directory to another location.
  - Backup the files and push to Google Drive.
  - Push the contents of the `drive-back-this-up` directory to Google Drive.

## Prerequisites

Before using `back-this-up`, ensure that you have the required dependencies installed:

- **Git:** For Git-related operations.
- **[gdrive](https://github.com/gdrive-org/gdrive):** For Google Drive-related operations.

## Usage

```shell
back-this-up [FLAGS]
Flags:

-g, --gp: Push the back-this-up directory to Git.
-b, --gb: Backup the files in the back-this-up directory.
-a, --gbp: Backup the files and push to Git.
-i, --dbp: Backup the files and push to Google Drive.
-s, --db: Backup the files in the drive-back-this-up directory.
-d, --dp: Push the drive-back-this-up directory to Google Drive.
Example Usage:

To backup files in the back-this-up directory to Git:

shell
Copy code
back-this-up -b
To push the contents of the drive-back-this-up directory to Google Drive:

shell
Copy code
back-this-up -d
Directory Structure
back-this-up manages two main directories:

back-this-up: Used for Git-related operations.
drive-back-this-up: Used for Google Drive-related operations.
Installation
Clone the repository to your local machine:

shell
Copy code
git clone https://github.com/Marlon-Sbardelatti/back-this-up.git
Navigate to the project directory:

shell
Copy code
cd back-this-up
Build the project:

shell
Copy code
cargo build --release
Find the binary in the target/release directory:

shell
Copy code
target/release/back-this-up
You can copy the binary to a directory in your PATH to run it from anywhere.

License
This project is licensed under the MIT License - see the LICENSE file for details.

css
Copy code

You can save this Markdown content to a file with a `.md` extension and it will serve as your README.



User
generate everything as .md
ChatGPT
Certainly! Here are the contents in Markdown format:

main.rs
markdown
Copy code
```rust
use std::fs;
use std::path::PathBuf;
mod git;
use clap::{Arg, Command};
mod drive;
use drive::functions::{drive_backup, drive_copy_dir, get_drive_current_dir};
use git::functions::{get_git_current_dir, git_backup, git_copy_dir};

fn main() {
    // Your Rust code here...
}
rust
Copy code

### `drive.rs`

```markdown
```rust
pub mod functions {
    extern crate fs_extra;
    use fs_extra::dir::{self, CopyOptions};
    use std::env;
    use std::path::PathBuf;
    use std::process::Command;

    pub fn drive_copy_dir(source: &PathBuf, destination: &PathBuf) {
        // Your drive_copy_dir function code here...
    }

    pub fn get_drive_current_dir() -> PathBuf {
        // Your get_drive_current_dir function code here...
    }

    pub fn drive_backup() {
        // Your drive_backup function code here...
    }
}
rust
Copy code

### `git.rs`

```markdown
```rust
pub mod functions {
    extern crate fs_extra;
    use fs_extra::dir::{self, CopyOptions};
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;

    pub fn git_copy_dir(source: &PathBuf, destination: &PathBuf) {
        // Your git_copy_dir function code here...
    }

    pub fn get_git_current_dir() -> PathBuf {
        // Your get_git_current_dir function code here...
    }

    pub fn git_backup() {
        // Your git_backup function code here...
    }
}
kotlin
Copy code

### `README.md`

```markdown
# back-this-up

**Version**: 1.0.0

A simple command-line tool to facilitate the backup and management of files. Designed for easy backup to both Git and Google Drive.

**Author**: Marlon Sbardelatti

## Overview

`back-this-up` is a command-line tool that simplifies the process of backing up files to both Git and Google Drive. It provides options to perform the following tasks:

- **Git Operations:**
  - Push the contents of the `back-this-up` directory to Git.
  - Backup the files in the `back-this-up` directory.
  - Backup the files and push to Git.

- **Google Drive Operations:**
  - Copy the contents of the `drive-back-this-up` directory to another location.
  - Backup the files and push to Google Drive.
  - Push the contents of the `drive-back-this-up` directory to Google Drive.

## Prerequisites




User
not the code, just provide me the readme as .md
ChatGPT
Certainly, here's the README in Markdown format:

markdown
Copy code
# back-this-up

**Version**: 1.0.0

A simple command-line tool to facilitate the backup and management of files. Designed for easy backup to both Git and Google Drive.

**Author**: Marlon Sbardelatti

## Overview

`back-this-up` is a command-line tool that simplifies the process of backing up files to both Git and Google Drive. It provides options to perform the following tasks:

- **Git Operations:**
  - Push the contents of the `back-this-up` directory to Git.
  - Backup the files in the `back-this-up` directory.
  - Backup the files and push to Git.

- **Google Drive Operations:**
  - Copy the contents of the `drive-back-this-up` directory to another location.
  - Backup the files and push to Google Drive.
  - Push the contents of the `drive-back-this-up` directory to Google Drive.

## Prerequisites

Before using `back-this-up`, ensure that you have the required dependencies installed:

- **Git:** For Git-related operations.
- **[gdrive](https://github.com/gdrive-org/gdrive):** For Google Drive-related operations.

## Usage

```shell
back-this-up [FLAGS]
Flags:

-g, --gp: Push the back-this-up directory to Git.
-b, --gb: Backup the files in the back-this-up directory.
-a, --gbp: Backup the files and push to Git.
-i, --dbp: Backup the files and push to Google Drive.
-s, --db: Backup the files in the drive-back-this-up directory.
-d, --dp: Push the drive-back-this-up directory to Google Drive.
Example Usage:

To backup files in the back-this-up directory to Git:

shell
Copy code
back-this-up -b
To push the contents of the drive-back-this-up directory to Google Drive:

shell
Copy code
back-this-up -d
Directory Structure
back-this-up manages two main directories:

back-this-up: Used for Git-related operations.
drive-back-this-up: Used for Google Drive-related operations.
Installation
Clone the repository to your local machine:

shell
Copy code
git clone https://github.com/Marlon-Sbardelatti/back-this-up.git
Navigate to the project directory:

shell
Copy code
cd back-this-up
Build the project:

shell
Copy code
cargo build --release
Find the binary in the target/release directory:

shell
Copy code
target/release/back-this-up
You can copy the binary to a directory in your PATH to run it from anywhere.

License
This project is licensed under the MIT License - see the LICENSE file for details.

css
Copy code

You can save this Markdown content to a file with a `.md` extension to use as your README.




