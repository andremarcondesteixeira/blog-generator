use std::path::{Path, PathBuf};

/// Validated command-line arguments for the blog generator.
///
/// Guarantees that the config file path is absolute, has a `.json` extension,
/// and exists on disk at the time of construction.
#[derive(Debug)]
pub struct Args {
    config_file_path: PathBuf,
}

impl Args {
    /// Creates a new Args instance with the given config file path.
    ///
    /// # Errors
    /// Returns an error if the path is not absolute, does not have a `.json`
    /// extension, or does not exist on disk.
    pub fn new(config_file_path: impl Into<PathBuf>) -> Result<Args, String> {
        let config_file_path: PathBuf = config_file_path.into();
        if !config_file_path.is_absolute() {
            return Err(format!("config file path must be absolute: {}", config_file_path.display()));
        }
        if config_file_path.extension().and_then(|e| e.to_str()) != Some("json") {
            return Err(format!("config file must be a .json file: {}", config_file_path.display()));
        }
        if !config_file_path.exists() {
            return Err(format!("config file does not exist: {}", config_file_path.display()));
        }
        Ok(Args { config_file_path })
    }

    /// Collects the command line arguments and use them to create a new Args instance.
    ///
    /// # Errors
    /// Returns an error if no command line argument was provided
    pub fn from_command_line_arguments() -> Result<Args, String> {
        let raw_args: Vec<String> = std::env::args().collect();
        if raw_args.len() < 2 {
            return Err("Usage: blog-generator <absolute path to config file>".to_string());
        }
        Args::new(&raw_args[1])
    }

    pub fn config_file_path(&self) -> &Path {
        &self.config_file_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_rejects_relative_config_path() {
        let err = Args::new("relative/my_blog_config.json").unwrap_err();
        assert!(err.contains("must be absolute"), "Unexpected error message: {err}");
    }

    #[test]
    fn args_accepts_absolute_config_path() {
        let dir = std::env::temp_dir();
        let path = dir.join("test_blog_config.json");
        std::fs::write(&path, "{}").unwrap();

        let args = Args::new(&path).unwrap();
        assert_eq!(args.config_file_path(), path);

        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn args_rejects_non_json_config_file() {
        let path = if cfg!(windows) {
            "C:\\absolute\\my_blog_config.toml"
        } else {
            "/absolute/my_blog_config.toml"
        };
        let err = Args::new(path).unwrap_err();
        assert!(err.contains("must be a .json file"), "Unexpected error message: {err}");
    }

    #[test]
    fn args_rejects_nonexistent_config_file() {
        let path = if cfg!(windows) {
            "C:\\nonexistent\\my_blog_config.json"
        } else {
            "/nonexistent/my_blog_config.json"
        };
        let err = Args::new(path).unwrap_err();
        assert!(err.contains("does not exist"), "Unexpected error message: {err}");
    }
}
