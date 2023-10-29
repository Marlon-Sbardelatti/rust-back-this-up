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
- **GitHub:** Create a repo in your github account After you run the cli tool for the first time and the directories are created make sure to add the origin to this repo in the directory.

## Usage

```shell
back-this-up [FLAGS]
```
# Flags:

- -g, --gp: Push the back-this-up directory to GitHub.
- -b, --gb: Backup the files in the back-this-up directory.
- -a, --gbp: Backup the files and push to GitHub.
- -i, --dbp: Backup the files and push to Google Drive.
- -s, --db: Backup the files in the drive-back-this-up directory.
- -d, --dp: Push the drive-back-this-up directory to Google Drive.

# Example Usage

- **To backup files in the back-this-up directory to Git**:

 
```shell
back-this-up -b
```

## Directory Structure
**back-this-up manages two main directories**:

- back-this-up: Used for Git-related operations.
- drive-back-this-up: Used for Google Drive-related operations.

## Installation
**1. Clone the repository to your local machine**:

```shell
git clone https://github.com/Marlon-Sbardelatti/back-this-up.git
```
**2. Navigate to the project directory**:

```shell
cd back-this-up
```
**3.Build the project**:

```shell
cargo build --release
```
**4.Find the binary in the target/release directory**:

```shell
target/release/back-this-up
```
**5.You can copy the binary to a directory in your PATH to run it from anywhere**.
```shell
sudo cp ./back-this-up /usr/local/bin
```

