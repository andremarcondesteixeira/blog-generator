pub mod core;
pub mod io;
pub use io::input::args::Args;
pub use core::blog_configuration::BlogConfiguration;

/// Generates all blogs from the given configurations.
///
/// # Errors
/// Returns an error if any blog generation fails.
pub fn run(_blog_configurations: Vec<BlogConfiguration>) -> Result<(), String> {
    Ok(())
}
