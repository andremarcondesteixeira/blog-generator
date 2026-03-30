pub mod core;
pub use core::args::Args;

pub fn run(_args: Args) -> Result<(), String> {
    Ok(())
}
