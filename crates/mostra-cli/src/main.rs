use std::env;
use std::process::ExitCode;

fn print_help() {
    println!(
        "Mostra {}\n\nLocal-first personal data exposure analysis.\n\nUSAGE:\n    mostra [OPTIONS]\n\nOPTIONS:\n    -h, --help       Print help\n    -V, --version    Print version",
        mostra::version()
    );
}

fn main() -> ExitCode {
    match env::args().nth(1).as_deref() {
        None | Some("-h" | "--help") => {
            print_help();
            ExitCode::SUCCESS
        }

        Some("-V" | "--version") => {
            println!("mostra {}", mostra::version());
            ExitCode::SUCCESS
        }

        Some(command) => {
            eprintln!("mostra: command not available yet: {command}");
            eprintln!("Run `mostra --help` for usage.");
            ExitCode::from(2)
        }
    }
}
