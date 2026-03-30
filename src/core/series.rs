use serde::Deserialize;

use super::article::Article;

/// Represents a series of related articles as declared in the input JSON configuration.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    /// The name of the series.
    pub name: String,
    /// Path segments to the series input folder, relative to the blog's input folder.
    pub input_folder: Vec<String>,
    /// Tags inherited by all articles in the series.
    pub tags: Vec<String>,
    /// Articles in the series, ordered by their position in the array.
    pub articles: Vec<Article>,
}

impl Series {
    /// Deserializes a Series from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_series_from_json() {
        let json = r#"{
            "name": "My Series",
            "inputFolder": ["path", "to", "series"],
            "tags": ["Series Tag"],
            "articles": [
                {
                    "title": "First Article",
                    "publicationDate": "2026-03-28",
                    "source": ["first.html"],
                    "tags": ["Tag A"]
                },
                {
                    "title": "Second Article",
                    "publicationDate": "2026-03-30",
                    "updateDate": "2026-04-01",
                    "source": ["second.html"],
                    "tags": ["Tag B"]
                }
            ]
        }"#;

        let series = Series::from_json(json).unwrap();

        assert_eq!(series.name, "My Series");
        assert_eq!(series.input_folder, vec!["path", "to", "series"]);
        assert_eq!(series.tags, vec!["Series Tag"]);
        assert_eq!(series.articles.len(), 2);
        assert_eq!(series.articles[0].title, "First Article");
        assert_eq!(series.articles[1].update_date, Some("2026-04-01".to_string()));
    }

    #[test]
    fn from_json_returns_error_as_string_on_invalid_json() {
        let result: Result<Series, String> = Series::from_json("not json");
        assert!(result.is_err());
    }
}