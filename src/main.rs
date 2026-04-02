use blog_generator::{run, Args};
use std::process;

fn main() {
    let _args = Args::from_command_line_arguments().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        process::exit(1);
    });

    // TODO: read config file, load templates and articles, build Vec<BlogConfiguration>
    let blog_configurations = vec![];

    if let Err(e) = run(blog_configurations) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
