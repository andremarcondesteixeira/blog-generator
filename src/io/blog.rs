use serde::Deserialize;

use super::article::Article;
use super::series::Series;

/// Represents a blog as declared in the input JSON configuration.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blog {
    /// The name of the blog.
    pub name: String,
    /// Path segments to the blog's input folder.
    pub input_folder: Vec<String>,
    /// Path segments to the blog's output folder.
    pub output_folder: Vec<String>,
    /// Path segments to the article template, relative to the blog's input folder.
    pub article_template: Vec<String>,
    /// Path segments to the index template, relative to the blog's input folder.
    pub index_template: Vec<String>,
    /// Standalone articles not belonging to any series.
    pub standalone_articles: Vec<Article>,
    /// Series of related articles.
    pub series: Vec<Series>,
}

impl Blog {
    /// Deserializes a Blog from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_blog_from_json() {
        let json = r#"{
            "name": "My Blog",
            "inputFolder": ["path", "to", "input"],
            "outputFolder": ["path", "to", "output"],
            "articleTemplate": ["templates", "article.html"],
            "indexTemplate": ["templates", "index.html"],
            "standaloneArticles": [
                {
                    "title": "Standalone Article",
                    "publicationDate": "2026-03-28",
                    "source": ["standalone.html"],
                    "tags": ["Tag A"]
                }
            ],
            "series": [
                {
                    "name": "My Series",
                    "inputFolder": ["series", "folder"],
                    "tags": ["Series Tag"],
                    "articles": [
                        {
                            "title": "Series Article",
                            "publicationDate": "2026-03-30",
                            "source": ["first.html"],
                            "tags": ["Tag B"]
                        }
                    ]
                }
            ]
        }"#;

        let blog = Blog::from_json(json).unwrap();

        assert_eq!(blog.name, "My Blog");
        assert_eq!(blog.input_folder, vec!["path", "to", "input"]);
        assert_eq!(blog.output_folder, vec!["path", "to", "output"]);
        assert_eq!(blog.article_template, vec!["templates", "article.html"]);
        assert_eq!(blog.index_template, vec!["templates", "index.html"]);
        assert_eq!(blog.standalone_articles.len(), 1);
        assert_eq!(blog.standalone_articles[0].title, "Standalone Article");
        assert_eq!(blog.series.len(), 1);
        assert_eq!(blog.series[0].name, "My Series");
    }

    #[test]
    fn from_json_returns_error_as_string_on_invalid_json() {
        let result: Result<Blog, String> = Blog::from_json("not json");
        assert!(result.is_err());
    }
}