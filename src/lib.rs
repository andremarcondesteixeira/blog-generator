pub mod core;
pub mod io;
pub use io::args::Args;

pub fn run(_args: Args) -> Result<(), String> {
    Ok(())
}
