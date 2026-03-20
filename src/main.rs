use std::process;

fn main() {
    let args = blog_generator::Args::from_command_line_arguments().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        process::exit(1);
    });
    
    if let Err(e) = blog_generator::run(&args) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
