use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Args {
    config_file: PathBuf,
}

impl Args {
    pub fn new(path: impl Into<PathBuf>) -> Result<Args, String> {
        let config_file: PathBuf = path.into();
        if !config_file.is_absolute() {
            return Err(format!("config file path must be absolute: {}", config_file.display()));
        }
        if config_file.extension().and_then(|e| e.to_str()) != Some("json") {
            return Err(format!("config file must be a .json file: {}", config_file.display()));
        }
        if !config_file.exists() {
            return Err(format!("config file does not exist: {}", config_file.display()));
        }
        Ok(Args { config_file })
    }

    pub fn parse_cli() -> Result<Args, String> {
        let raw_args: Vec<String> = std::env::args().collect();
        if raw_args.len() < 2 {
            return Err("Usage: blog-generator <absolute path to config file>".to_string());
        }
        Args::new(&raw_args[1])
    }

    pub fn config_file(&self) -> &Path {
        &self.config_file
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_rejects_relative_config_path() {
        let err = Args::new("relative/my_blog_config.json").unwrap_err();
        assert!(err.contains("must be absolute"), "unexpected error: {err}");
    }

    #[test]
    fn args_accepts_absolute_config_path() {
        let dir = std::env::temp_dir();
        let path = dir.join("test_blog_config.json");
        std::fs::write(&path, "{}").unwrap();

        let args = Args::new(&path).unwrap();
        assert_eq!(args.config_file(), path);

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
        assert!(err.contains("must be a .json file"), "unexpected error: {err}");
    }

    #[test]
    fn args_rejects_nonexistent_config_file() {
        let path = if cfg!(windows) {
            "C:\\nonexistent\\my_blog_config.json"
        } else {
            "/nonexistent/my_blog_config.json"
        };
        let err = Args::new(path).unwrap_err();
        assert!(err.contains("does not exist"), "unexpected error: {err}");
    }
}
