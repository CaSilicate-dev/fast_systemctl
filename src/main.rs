use colored::*;
use std::env;
use std::process::Command;

pub fn print_help() {
    let help_str = include_str!("../assets/help_message.txt");

    print!("{}", help_str);
}

fn run_systemctl(sub_command: String, left_args: String) {
    let command = format!("systemctl {} {}", sub_command, left_args);

    println!("{} {}", "Executing: ".green(), command.bright_green());

    Command::new("bash")
        .arg("-c")
        .arg(command)
        .status()
        .expect("failed to execute process");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let length = args.len() - 1;
    if length < 1 {
        eprintln!(
            "{}",
            format!("invalid number of arguments").red()
        );
        print_help();
        std::process::exit(1);
    }

    let raw_sub_command = args.get(1).expect("missing systemctl subcommand");
    let sctl_sub_cmd: String;

    match raw_sub_command.as_str() {
        "l" => sctl_sub_cmd = "start".to_string(),
        "p" => sctl_sub_cmd = "stop".to_string(),
        "r" => sctl_sub_cmd = "restart".to_string(),
        "t" => sctl_sub_cmd = "status".to_string(),

        "e" => sctl_sub_cmd = "enable".to_string(),
        "ue" => sctl_sub_cmd = "disable".to_string(),
        "m" => sctl_sub_cmd = "mask".to_string(),
        "um" => sctl_sub_cmd = "unmask".to_string(),

        "dr" => sctl_sub_cmd = "daemon-reload".to_string(),
        "drx" => sctl_sub_cmd = "daemon-reexec".to_string(),
        "ls" => sctl_sub_cmd = "list-units".to_string(),
        "lf" => sctl_sub_cmd = "list-unit-files".to_string(),

        _ => {
            sctl_sub_cmd = String::from(raw_sub_command);
        }
    }

    let mut left_str = "".to_string();

    for a in args.iter().skip(2) {
        left_str.push_str(a.as_str());
        left_str.push_str(" ");
    }

    run_systemctl(sctl_sub_cmd, left_str);
}
