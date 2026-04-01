use serde::Deserialize;

use crate::io::input::blog::Blog;

/// Represents the root structure of the input JSON configuration file.
#[derive(Deserialize)]
pub struct InputConfiguration {
    /// The list of blogs to generate.
    pub blogs: Vec<Blog>,
}

impl InputConfiguration {
    /// Deserializes an InputConfiguration from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_input_configuration_from_json() {
        let json = r#"{
            "blogs": [
                {
                    "name": "My Blog",
                    "inputFolder": ["input"],
                    "outputFolder": ["output"],
                    "articleTemplate": ["template.html"],
                    "indexTemplate": ["index.html"],
                    "standaloneArticles": [],
                    "series": []
                }
            ]
        }"#;

        let config = InputConfiguration::from_json(json).unwrap();

        assert_eq!(config.blogs.len(), 1);
        assert_eq!(config.blogs[0].name, "My Blog");
    }

    #[test]
    fn from_json_returns_error_as_string_on_invalid_json() {
        let result: Result<InputConfiguration, String> = InputConfiguration::from_json("not json");
        assert!(result.is_err());
    }
}
