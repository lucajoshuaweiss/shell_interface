//! Functions that allow the user to run shell operations

use std::process::Stdio;
use std::process::Command;
use gtk::Label;

/// Lets the user run a shell operation. A status label will be updated to the stdout of the operation
pub fn operation_with_status(status: &Label, shell: &str, shell_option:&str, command: &str){
    let operation = Command::new(shell).arg(shell_option)
        .arg(command)
        .stdout(Stdio::piped())
        .output()
        .expect(&format!("ERROR in operation_with_status(), is your system using '{shell}', does it support the option '{shell_option}' and has it '{command}' installed?"));

    let stdout = String::from_utf8(operation.stdout).unwrap();
    status.set_text(&stdout);
}


/// Lets the user run a shell operation. A status label will be updated with a string
pub fn operation_with_simplified_status(status: &Label, shell: &str, shell_option:&str, command: &str){
    let _operation = Command::new(shell).arg(shell_option)
        .arg(command)
        .stdout(Stdio::piped())
        .output()
        .expect(&format!("ERROR in operation_with_simplified_status(), is your system using '{shell}', does it support the option '{shell_option}' and has it '{command}' installed?"));

    status.set_text("Operation is done");
}

/// Lets the user run a shell operation
pub fn operation(shell: &str, shell_option: &str, command: &str){
    let _operation = Command::new(shell).arg(shell_option)
        .arg(command)
        .stdout(Stdio::piped())
        .output()
        .expect(&format!("ERROR in operation(), is your system using '{shell}', does it support the option '{shell_option}' and has it '{command}' installed?"));
}
