/// A validated absolute URL path relative to the blog root URL.
///
/// Segments are separated by `/`. All segments except the last may only
/// contain alphanumeric characters (`a-z A-Z 0-9`), hyphens (`-`), and
/// underscores (`_`). The last segment may additionally contain periods
/// (`.`) to allow file extensions.
///
/// For example, `"posts/my-series/part-1/article.html"` is valid, but
/// `"posts/my-series.pdf/article.html"` is not.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UrlPath {
    value: String,
}

impl UrlPath {
    /// Creates a new UrlPath from the given string.
    ///
    /// # Errors
    /// Returns an error if any segment is empty, if the path is empty,
    /// if non-last segments contain characters other than `a-z A-Z 0-9 - _`,
    /// or if the last segment contains characters other than `a-z A-Z 0-9 - _ .`.
    pub fn new(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err("URL path must not be empty".to_string());
        }
        let segments: Vec<&str> = value.split('/').collect();
        for segment in &segments {
            if segment.is_empty() {
                return Err("URL path segment must not be empty".to_string());
            }
        }
        for (index, segment) in segments.iter().enumerate() {
            let is_last_path_segment = index == segments.len() - 1;
            for character in segment.chars() {
                let is_valid = character.is_ascii_alphanumeric()
                    || character == '-'
                    || character == '_'
                    || (is_last_path_segment && character == '.');
                if !is_valid {
                    return Err(format!(
                        "invalid character '{character}' in URL path segment \"{segment}\""
                    ));
                }
            }
        }
        Ok(UrlPath { value })
    }

    /// Returns the URL path as a string.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty_path() {
        let error = UrlPath::new("".to_string()).unwrap_err();
        assert!(
            error.contains("must not be empty"),
            "Unexpected error message: {error}"
        );
    }

    #[test]
    fn new_rejects_empty_segment() {
        let error = UrlPath::new("posts//article.html".to_string()).unwrap_err();
        assert!(
            error.contains("must not be empty"),
            "Unexpected error message: {error}"
        );
    }

    #[test]
    fn new_rejects_period_in_non_last_segment() {
        let error = UrlPath::new(
            "posts/my-series.pdf/article.html".to_string(),
        )
        .unwrap_err();
        assert!(
            error.contains("invalid character"),
            "Unexpected error message: {error}"
        );
    }

    #[test]
    fn new_rejects_invalid_character_in_last_segment() {
        let error = UrlPath::new("posts/article#1.html".to_string()).unwrap_err();
        assert!(
            error.contains("invalid character"),
            "Unexpected error message: {error}"
        );
    }

    #[test]
    fn new_rejects_leading_slash() {
        let error = UrlPath::new("/posts/article.html".to_string()).unwrap_err();
        assert!(
            error.contains("must not be empty"),
            "Unexpected error message: {error}"
        );
    }

    #[test]
    fn new_rejects_trailing_slash() {
        let error = UrlPath::new("posts/article.html/".to_string()).unwrap_err();
        assert!(
            error.contains("must not be empty"),
            "Unexpected error message: {error}"
        );
    }

    #[test]
    fn new_creates_url_path_from_valid_path() {
        let url_path = UrlPath::new(
            "posts/my-series/part-1/article.html".to_string(),
        )
        .unwrap();

        assert_eq!(
            url_path.as_str(),
            "posts/my-series/part-1/article.html"
        );
    }
}