use std::env;
use std::os::unix::process::CommandExt;
use std::process::{self, Command};

const PROG_NAME: &str = "fromdir";

fn usage(prog: &str) {
    eprintln!("Usage: {prog} <directory> <command> [args...]");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let prog_name = args
        .first()
        .map_or(PROG_NAME.to_string(), std::clone::Clone::clone);

    if args.len() < 2 {
        usage(&prog_name);
        process::exit(1);
    }

    let dir_path = &args[1];

    if args.len() < 3 {
        eprintln!("Error: No command specified");
        usage(&prog_name);
        process::exit(1);
    }

    if let Err(err) = env::set_current_dir(dir_path) {
        eprintln!("Error: Failed to change directory to {dir_path:?}: {err}");
        process::exit(1);
    }

    let cmd = &args[2];
    let cmd_args = args.iter().skip(3).collect::<Vec<&String>>();

    let mut command = Command::new(cmd);
    command.args(&cmd_args);

    // This will *only* return if exec fails.
    let err = command.exec();
    eprintln!("Error: Failed to execute {cmd:?}: {err}");
    process::exit(1);
}
