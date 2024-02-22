use clap::{Arg, Command};
use std::{fs::{self, File, OpenOptions}, io::{Read, Write}, process::Command as ProcessCommand};
use std::path::{Path, PathBuf};
use regex::Regex;

const SCRIPT_HEADER: &str = r#"
#!/bin/sh
#     ___               _                        ___   __   _____ 
#    / __\ __ _  _ __  | |_  _   _  _ __  ___   / __\ / /   \_   \
#   / /   / _` || '_ \ | __|| | | || '__|/ _ \ / /   / /     / /\/
#  / /___| (_| || |_) || |_ | |_| || |  |  __// /___/ /___/\/ /_  
#  \____/ \__,_|| .__/  \__| \__,_||_|   \___|\____/\____/\____/  
#               |_| 
#
# Script generated using CaptureCLI by coderipper
# Website: https://capturecli.xyz
#
# CaptureSettings:
"#;

fn main() {
    let matches = Command::new("CaptureCLI")
        .version("0.1.0")
        .author("coderipper <hello@joaquinsoza.dev>")
        .about("Captures commands and saves them to a script file for later use")
        .arg(Arg::new("script")
            .help("The name of the script file or the command to capture")
            .required(false)
            .index(1)) // First positional argument for script name or command
        .arg(Arg::new("command")
            .help("The command to capture")
            .required(false)
            .index(2) // Second positional argument for command
            .num_args(1..)) // Accepts one or more values
        .subcommand(Command::new("new")
            .about("Creates a new script file for capturing commands")
            .arg(Arg::new("name")
                .help("The name of the script file")
                .required(true)
                .index(1)))
        .subcommand(Command::new("config")
            .about("Add or edit configuration for the script file")
            .arg(Arg::new("name")
                .help("The name of the script file")
                .required(true)
                .index(1)))
        .subcommand(Command::new("list")
            .about("Lists all the captured script files"))
        .get_matches();

    let home_dir = dirs::home_dir().unwrap().join("CaptureCLI");
    if !home_dir.exists() {
        fs::create_dir_all(&home_dir).unwrap();
    }

    if let Some(subcommand) = matches.subcommand() {
        match subcommand {
            ("new", sub_m) => {
                let name = sub_m.get_one::<String>("name").unwrap();
                let file_path = create_new_script_file(name, &home_dir);
                config_script_file(&file_path);
            },
            ("list", _) => {
                let entries = fs::read_dir(home_dir).unwrap();
                for entry in entries {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "sh") {
                        println!("{}", path.file_name().unwrap().to_string_lossy());
                    }
                }
            },
            ("config", sub_m) => {
                let name = sub_m.get_one::<String>("name").unwrap();
                let file_path = create_new_script_file(name, &home_dir);
                config_script_file(&file_path);
            },
            _ => unreachable!("Exhaustive match of subcommands"),
        }
    } else {
        // Adjusted to handle multiple command arguments
        let script_name = matches.get_one::<String>("script").map_or("default", String::as_str);
        
        // Assuming 'command' now correctly handles multiple values due to .num_args(1..)
        if let Some(command_values) = matches.get_many::<String>("command") {
            let command_to_run = command_values.map(|s| s.as_str()).collect::<Vec<&str>>().join(" ");
            let file_path = create_new_script_file(script_name, &home_dir);
            append_command_to_script(&file_path, &command_to_run);
        
            // Execute the command
            let status = ProcessCommand::new("sh")
                .arg("-c")
                .arg(&command_to_run)
                .status() // Use .status() instead of .output()
                .expect("Failed to execute command");
        
            if !status.success() {
                eprintln!("Command failed to execute properly");
            }
        
            println!("Command executed and captured: {}", command_to_run);
        }        
    }
}

fn create_new_script_file(name: &str, home_dir: &Path) -> PathBuf {
    let file_path = home_dir.join(format!("{}.sh", name));

    // Check if the file already exists
    if file_path.exists() {
        let mut file_content = String::new();
        // Read the existing content to check if the header is present
        fs::File::open(&file_path)
            .unwrap()
            .read_to_string(&mut file_content)
            .unwrap();

        // Check for a unique part of the header to determine if it's already there
        if !file_content.contains("# Script generated using CaptureCLI") {
            // If the header is not found, append it
            let mut file = OpenOptions::new().append(true).open(&file_path).unwrap();
            writeln!(file, "{}", SCRIPT_HEADER).unwrap();
        }
    } else {
        // If the file does not exist, create it and add the header
        let mut file = OpenOptions::new().create(true).write(true).open(&file_path).unwrap();
        writeln!(file, "{}", SCRIPT_HEADER).unwrap();
    }

    println!("Created or updated script file: {}", name);
    file_path
}


fn config_script_file(file_path: &Path) {
    // Read the current content of the file
    let mut file_content = String::new();
    let mut file = File::open(file_path).unwrap();
    file.read_to_string(&mut file_content).unwrap();

    // Ask user for new setting
    println!("Do you want to be prompted for comments before each command? [y/N]: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let enable_step_comments = matches!(input.trim().to_lowercase().as_str(), "y" | "yes");

    // Prepare the new configuration line
    let new_config_line = format!("# - enable_step_comments={}\n", enable_step_comments);

    // Check if "CaptureSettings" exists
    if file_content.contains("CaptureSettings:") {
        // Regex to find and replace the existing setting line
        let re = Regex::new(r"(?m)^# - enable_step_comments=(true|false)$").unwrap();
        if re.is_match(&file_content) {
            // Replace existing line
            let updated_content = re.replace(&file_content, &new_config_line[..new_config_line.len() - 1]).to_string();
            fs::write(file_path, updated_content).unwrap();
        } else {
            // Append new setting under "CaptureSettings"
            let updated_content = file_content.replace("CaptureSettings:", &format!("CaptureSettings:\n{}", &new_config_line));
            fs::write(file_path, updated_content).unwrap();
        }
    } else {
        // If "CaptureSettings" section doesn't exist, append it along with the new setting
        let updated_content = format!("{}\nCaptureSettings:\n{}", file_content, new_config_line);
        fs::write(file_path, updated_content).unwrap();
    }
}

fn append_command_to_script(script_path: &Path, command: &str) {
    // Read the script file to check if ask_for_comments is enabled
    let content = std::fs::read_to_string(script_path).unwrap();
    let enable_step_comments = content.contains("# - enable_step_comments=true");
    
    let mut file = OpenOptions::new().append(true).open(script_path).unwrap();
    
    if enable_step_comments {
        println!("Enter a comment for this step (press Enter to skip): ");
        let mut comment = String::new();
        std::io::stdin().read_line(&mut comment).unwrap();
        
        if !comment.trim().is_empty() {
            writeln!(file, "echo -e \"\\033[0;32m{}\\033[0m\"", comment.trim()).unwrap();
        }
    }
    
    writeln!(file, "{}", command).unwrap();
}
