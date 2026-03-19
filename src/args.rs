use std::path::{Path, PathBuf};

pub struct Args {
    config_file: PathBuf,
}

impl Args {
    pub fn new(path: impl Into<PathBuf>) -> Result<Args, String> {
        let config_file: PathBuf = path.into();
        if !config_file.is_absolute() {
            return Err(format!("config file path must be absolute: {}", config_file.display()));
        }
        Ok(Args { config_file })
    }

    pub fn parse_cli() -> Result<Args, String> {
        let raw_args: Vec<String> = std::env::args().collect();
        if raw_args.len() < 2 {
            return Err("Usage: blog-generator <config-file>".to_string());
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
    use std::path::PathBuf;

    #[test]
    fn args_rejects_relative_config_path() {
        let result = Args::new("relative/my_blog_config.json");
        assert!(result.is_err());
    }

    #[test]
    fn args_accepts_absolute_config_path() {
        let path = if cfg!(windows) {
            "C:\\absolute\\my_blog_config.json"
        } else {
            "/absolute/my_blog_config.json"
        };
        let args = Args::new(path).unwrap();
        assert_eq!(args.config_file(), PathBuf::from(path));
    }
}
