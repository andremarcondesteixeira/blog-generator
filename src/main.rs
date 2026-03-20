use blog_generator::{run, Args};
use std::process;

fn main() {
    let args = Args::from_command_line_arguments().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        process::exit(1);
    });

    if let Err(e) = run(args) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
