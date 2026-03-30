use serde::Deserialize;

use super::blog::Blog;

/// Represents the root structure of the input JSON configuration file.
#[derive(Deserialize)]
pub struct InputConfiguration {
    /// The list of blogs to generate.
    pub blogs: Vec<Blog>,
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

        let config: InputConfiguration = serde_json::from_str(json).unwrap();

        assert_eq!(config.blogs.len(), 1);
        assert_eq!(config.blogs[0].name, "My Blog");
    }
}