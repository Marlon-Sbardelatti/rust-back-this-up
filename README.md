## back-this-up
Version: 1.0.0

A simple command-line tool to facilitate the backup and management of files. Designed for easy backup to both Git and Google Drive.

Author: Marlon Sbardelatti

Overview
back-this-up is a command-line tool that simplifies the process of backing up files to both Git and Google Drive. It provides options to perform the following tasks:

Git Operations:

Push the contents of the back-this-up directory to Git.
Backup the files in the back-this-up directory.
Backup the files and push to Git.
Google Drive Operations:

Copy the contents of the drive-back-this-up directory to another location.
Backup the files and push to Google Drive.
Push the contents of the drive-back-this-up directory to Google Drive.
Prerequisites
Before using back-this-up, ensure that you have the required dependencies installed:

Git: For Git-related operations.
gdrive: For Google Drive-related operations.
Usage
shell
Copy code
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

