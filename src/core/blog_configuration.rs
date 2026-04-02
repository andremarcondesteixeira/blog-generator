use std::collections::BTreeMap;

use crate::core::url_path::UrlPath;

/// The configuration for generating a single blog, containing all
/// template content and article content needed for generation.
#[derive(Debug)]
pub struct BlogConfiguration {
    article_template: String,
    index_template: String,
    standalone_articles: BTreeMap<UrlPath, String>,
    series_articles: BTreeMap<UrlPath, BTreeMap<UrlPath, String>>,
}

impl BlogConfiguration {
    /// Creates a new BlogConfiguration from the given template content
    /// and article content.
    ///
    /// # Errors
    /// Returns an error if any of the inputs are invalid.
    pub fn new(
        article_template: String,
        index_template: String,
        standalone_articles: BTreeMap<UrlPath, String>,
        series_articles: BTreeMap<UrlPath, BTreeMap<UrlPath, String>>,
    ) -> Result<Self, String> {
        if article_template.is_empty() {
            return Err("article template must not be empty".to_string());
        }
        if index_template.is_empty() {
            return Err("index template must not be empty".to_string());
        }
        for (url_path, content) in &standalone_articles {
            if content.is_empty() {
                return Err(format!(
                    "article content must not be empty: {}", url_path.as_str()
                ));
            }
        }
        for (_series_path, articles) in &series_articles {
            for (url_path, content) in articles {
                if content.is_empty() {
                    return Err(format!(
                        "article content must not be empty: {}", url_path.as_str()
                    ));
                }
            }
        }
        Ok(BlogConfiguration {
            article_template,
            index_template,
            standalone_articles,
            series_articles,
        })
    }

    /// Returns the article template content.
    pub fn article_template(&self) -> &str {
        &self.article_template
    }

    /// Returns the index template content.
    pub fn index_template(&self) -> &str {
        &self.index_template
    }

    /// Returns the standalone articles map, keyed by URL path.
    pub fn standalone_articles(&self) -> &BTreeMap<UrlPath, String> {
        &self.standalone_articles
    }

    /// Returns the series articles map, keyed by series name,
    /// with each value being an ordered map of URL path to article content.
    pub fn series_articles(&self) -> &BTreeMap<UrlPath, BTreeMap<UrlPath, String>> {
        &self.series_articles
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::core::url_path::UrlPath;

    #[test]
    fn new_rejects_empty_series_article_content() {
        let series_url_path = UrlPath::new("my-series".to_string()).unwrap();
        let article_url_path = UrlPath::new("my-series/first-post.html".to_string()).unwrap();
        let mut series_inner = BTreeMap::new();
        series_inner.insert(article_url_path, "".to_string());
        let mut series_articles = BTreeMap::new();
        series_articles.insert(series_url_path, series_inner);

        let error = BlogConfiguration::new(
            "article template".to_string(),
            "index template".to_string(),
            BTreeMap::new(),
            series_articles,
        )
        .unwrap_err();
        assert!(
            error.contains("article content must not be empty"),
            "Unexpected error message: {error}"
        );
    }

    #[test]
    fn new_rejects_empty_standalone_article_content() {
        let url_path = UrlPath::new("article.html".to_string()).unwrap();
        let mut standalone_articles = BTreeMap::new();
        standalone_articles.insert(url_path, "".to_string());

        let error = BlogConfiguration::new(
            "article template".to_string(),
            "index template".to_string(),
            standalone_articles,
            BTreeMap::new(),
        )
        .unwrap_err();
        assert!(
            error.contains("article content must not be empty"),
            "Unexpected error message: {error}"
        );
    }

    #[test]
    fn new_rejects_empty_index_template() {
        let url_path = UrlPath::new("article.html".to_string()).unwrap();
        let mut standalone_articles = BTreeMap::new();
        standalone_articles.insert(url_path, "content".to_string());

        let error = BlogConfiguration::new(
            "article template".to_string(),
            "".to_string(),
            standalone_articles,
            BTreeMap::new(),
        )
        .unwrap_err();
        assert!(
            error.contains("index template must not be empty"),
            "Unexpected error message: {error}"
        );
    }

    #[test]
    fn new_rejects_empty_article_template() {
        let url_path = UrlPath::new("article.html".to_string()).unwrap();
        let mut standalone_articles = BTreeMap::new();
        standalone_articles.insert(url_path, "content".to_string());

        let error = BlogConfiguration::new(
            "".to_string(),
            "index template".to_string(),
            standalone_articles,
            BTreeMap::new(),
        )
        .unwrap_err();
        assert!(
            error.contains("article template must not be empty"),
            "Unexpected error message: {error}"
        );
    }

    #[test]
    fn new_creates_blog_configuration_with_valid_inputs() {
        let article_template = "article template content".to_string();
        let index_template = "index template content".to_string();

        let standalone_url_path = UrlPath::new("my-article.html".to_string()).unwrap();
        let mut standalone_articles = BTreeMap::new();
        standalone_articles.insert(
            standalone_url_path,
            "standalone article content".to_string(),
        );

        let series_article_url_path = UrlPath::new("series/first-post.html".to_string()).unwrap();
        let mut first_series_articles = BTreeMap::new();
        first_series_articles.insert(
            series_article_url_path,
            "first post content".to_string(),
        );
        let series_url_path = UrlPath::new("my-series".to_string()).unwrap();
        let mut series_articles = BTreeMap::new();
        series_articles.insert(series_url_path, first_series_articles);

        let blog_configuration = BlogConfiguration::new(
            article_template,
            index_template,
            standalone_articles,
            series_articles,
        )
        .unwrap();

        assert_eq!(
            blog_configuration.article_template(),
            "article template content"
        );
        assert_eq!(
            blog_configuration.index_template(),
            "index template content"
        );
        let standalone_key = UrlPath::new("my-article.html".to_string()).unwrap();
        assert_eq!(
            blog_configuration
                .standalone_articles()
                .get(&standalone_key)
                .unwrap(),
            "standalone article content"
        );
        let series_lookup_key = UrlPath::new("my-series".to_string()).unwrap();
        let series = blog_configuration
            .series_articles()
            .get(&series_lookup_key)
            .unwrap();
        let series_key = UrlPath::new("series/first-post.html".to_string()).unwrap();
        assert_eq!(series.get(&series_key).unwrap(), "first post content");
    }
}