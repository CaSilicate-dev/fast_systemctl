use colored::*;
use std::env;
use std::process::Command;



fn run_systemctl(subcmd: String, serivce: String) {
    let command = format!("systemctl {} {}", subcmd, serivce);

    println!("Executing: {}", command);

    Command::new("bash")
        .arg("-c")
        .arg(command)
        .status()
        .expect("failed to execute process");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let length = args.len() - 1;
    if length != 2 {
        println!(
            "{}",
            format!("invalid number of arguments, required 2, but you provided {length}").red()
        );
        std::process::exit(1);
    }

    let raw_subcmd = args.get(1).expect("missing systemctl subcommand");
    let service_name = args
        .get(2)
        .expect("missing systemctl service name")
        .to_string();
    let sctl_subcmd: String;

    match raw_subcmd.as_str() {
        "l" => sctl_subcmd = "start".to_string(),
        "r" => sctl_subcmd = "restart".to_string(),
        "t" => sctl_subcmd = "status".to_string(),
        "p" => sctl_subcmd = "stop".to_string(),
        "dr" => sctl_subcmd = "daemon-reload".to_string(),
        "drx" => sctl_subcmd = "daemon-reexec".to_string(),
        _ => {
            sctl_subcmd = String::from(raw_subcmd);
        }
    }

    run_systemctl(sctl_subcmd, service_name);
}
