use std::env;
use std::os::unix::process::CommandExt;
use std::process::{self, Command};

fn usage(prog_name: &str) {
    eprintln!("Usage: {prog_name} <directory> <command> [args...]");
}

fn main() {
    let mut args = env::args();

    let prog_name = args.next();
    let prog_name: &str = prog_name.as_deref().unwrap_or(env!("CARGO_PKG_NAME"));

    match (args.next(), args.next()) {
        (None, _) => {
            eprintln!("{prog_name}: error: No directory specified");
            usage(prog_name);
            process::exit(1);
        }
        (Some(_), None) => {
            eprintln!("{prog_name}: error: No command specified");
            usage(prog_name);
            process::exit(1);
        }
        (Some(dir_path), Some(cmd)) => {
            if let Err(err) = env::set_current_dir(&dir_path) {
                eprintln!("{prog_name}: error: Failed to change directory to {dir_path:?}: {err}");
                process::exit(1);
            }

            let mut command = Command::new(&cmd);
            command.args(args);

            // Execute command - this will only return if exec fails
            let err = command.exec();
            eprintln!("{prog_name}: error: Failed to execute {cmd:?}: {err}");
            process::exit(1);
        }
    }
}
