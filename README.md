# CaptureCLI

CaptureCLI is a command-line tool designed to streamline the process of capturing and automating shell commands. It allows users to easily save commands executed in the shell into script files for later use, making it an invaluable tool for setting up new environments, automating repetitive tasks, and documenting command sequences. Whether you're installing software, configuring settings, or running diagnostics, CaptureCLI simplifies the process by capturing your commands in an executable script.

## Features

- **Capture Commands**: Save any shell command into a script file with a simple prefix.
- **Automated Script Creation**: Automatically creates script files with user-defined names.
- **Configuration Options**: Customize behavior with script-level settings, such as prompting for command descriptions.
- **Easy Management**: List and configure scripts directly from the CLI.

## Getting Started

### Prerequisites

- Rust programming language (latest stable version recommended)
- Cargo (Rust's package manager and build system)

### Building from Source

1. Clone the repository:

   ```sh
   git clone https://github.com/joaquinsoza/CaptureCLI.git
   cd CaptureCLI
   ```

2. Build the project using Cargo:

   ```sh
   cargo build --release
   ```

3. The executable will be located in `./target/release/`.

### Running CaptureCLI

To run CaptureCLI directly through Cargo:

```sh
cargo run -- <arguments>
```

For example, to capture a new command:

```sh
cargo run -- newScript echo "Hello, World!"
```

### Installing CaptureCLI

After building, you can install CaptureCLI on your system:

```sh
cargo install --path .
```

This will install the executable, making it available as `capture` in your shell.

## Usage

- **Creating a New Script File**:
  ```sh
  capture new myscript
  ```
- **Adding Commands to a Script File**:
  ```sh
  capture myscript <command>
  ```
- **Listing All Scripts**:
  ```sh
  capture list
  ```
- **Configuring a Script File**:
  ```sh
  capture config myscript
  ```

## Script Storage Location

CaptureCLI organizes and stores all the script files in a specific directory within the user's home directory. This dedicated storage approach helps in managing and accessing scripts efficiently. Below are the details regarding the script storage location and how users can interact with their scripts.

#### Default Storage Directory

By default, all script files created or managed by CaptureCLI are saved in the `~/CaptureCLI/` directory. Each script file has a `.sh` extension, making it easy to recognize and execute in Unix-like environments. This directory structure not only helps in keeping your scripts organized but also separates them from other files for better management.

## Collaboration

CaptureCLI is an open-source project, and contributions are warmly welcomed. Whether you're reporting bugs, suggesting new features, or contributing code, your input is valuable. Please feel free to fork the repository, make your changes, and submit a pull request.

For more detailed information on contributing, please refer to CONTRIBUTING.md.
