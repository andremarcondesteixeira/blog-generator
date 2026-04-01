pub mod core;
pub mod io;
pub use io::input::args::Args;

pub fn run(_args: Args) -> Result<(), String> {
    Ok(())
}
