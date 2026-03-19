use std::process;

fn main() {
    let args = blog_generator::Args::parse_cli().unwrap_or_else(|e| {
        eprintln!("error: {e}");
        process::exit(1);
    });
    if let Err(e) = blog_generator::run(&args) {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
