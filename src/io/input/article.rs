use serde::Deserialize;

use crate::core::date::Date;

/// Represents an article as declared in the input JSON configuration.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    /// The article title, also used for ordering after publication date.
    pub title: String,
    /// The publication date.
    pub publication_date: Date,
    /// The date the article was last updated. None if never updated.
    pub update_date: Option<Date>,
    /// Path segments to the article source file, relative to the blog's input folder.
    pub source: Vec<String>,
    /// Free-form tags associated with this article.
    pub tags: Vec<String>,
}

impl Article {
    /// Deserializes an Article from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_standalone_article_with_all_fields_from_json() {
        let json = r#"{
            "title": "My Article",
            "publicationDate": "2026-03-28",
            "updateDate": "2026-03-29",
            "source": ["path", "to", "article.html"],
            "tags": ["Tag A", "Tag B"]
        }"#;

        let article = Article::from_json(json).unwrap();

        assert_eq!(article.title, "My Article");
        assert_eq!(article.publication_date, Date::from_year_month_day(2026, 3, 28).unwrap());
        assert_eq!(article.update_date, Some(Date::from_year_month_day(2026, 3, 29).unwrap()));
        assert_eq!(article.source, vec!["path", "to", "article.html"]);
        assert_eq!(article.tags, vec!["Tag A", "Tag B"]);
    }

    #[test]
    fn from_json_returns_error_as_string_on_invalid_json() {
        let result: Result<Article, String> = Article::from_json("not json");
        assert!(result.is_err());
    }
}
