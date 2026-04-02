# ADR-004: Input JSON Format

## Date
2026-03-30

## Status
Accepted

## Context
The blog generator needs a structured input file that describes every blog it should build: where to find sources, where to write output, which templates to use, and which articles to include.

A single JSON file keeps configuration centralized and easy to validate at startup.

## Decision
The input file uses the following JSON structure:

```json
{
  "blogs": [
    {
      "name": "The Blog's Name",
      "inputFolder": ["path", "to", "input", "folder"],
      "outputFolder": ["path", "to", "output", "folder"],
      "articleTemplate": ["path", "to", "template", "relative to the blog's input folder"],
      "indexTemplate": ["path", "to", "template", "relative to the blog's input folder", "where both standalone articles and series' articles are shown together ordered first by publication date descending (newer first), then by name ascending"],
      "standaloneArticles": [
        {
          "title": "The article title, used for ordering after publication date",
          "publicationDate": "2026-03-28",
          "updateDate": "2026-03-29",
          "source": ["path", "to", "article", "relative to the blog's input folder", "the slug is this source path.html"],
          "tags": ["Tag A", "Tag B"]
        },
        {
          "title": "The article title, used for ordering after publication date",
          "publicationDate": "2026-03-30",
          "source": ["path", "to", "article", "relative to the blog's input folder", "the slug is this source path.html"],
          "tags": ["Tag A", "Tag c"]
        }
      ],
      "series": [
        {
          "name": "The name of the series where articles are ordered by order of appearance in the array below",
          "inputFolder": ["path", "to", "input", "folder", "relative to the blog's input folder"],
          "tags": ["all", "articles", "in", "the", "series", "inherit", "these", "tags"],
          "articles": [
            {
              "title": "The article name",
              "publicationDate": "2026-03-30",
              "source": ["path", "to", "article", "relative to the serie's input folder", "the slug is the series name followed by this source path.html"],
              "tags": ["Tag C", "Tag D"]
            },
            {
              "title": "The article name",
              "publicationDate": "2026-03-30",
              "updateDate": "2026-04-01",
              "source": ["path", "to", "article", "relative to the serie's input folder", "the slug is the series name followed by this source path.html"],
              "tags": ["Foo", "Bar Baz Qux"]
            }
          ]
        }
      ]
    }
  ]
}
```

### Path conventions

- All paths are arrays of segments, making them platform-independent.
- `inputFolder` and `outputFolder` on the blog are relative to the directory containing the JSON configuration file.
- `articleTemplate` and `indexTemplate` are relative to the blog's `inputFolder`.
- Standalone article `source` paths are relative to the blog's `inputFolder`.
- Series `inputFolder` is relative to the blog's `inputFolder`.
- Series article `source` paths are relative to the series' `inputFolder`.

### Slug derivation

- Standalone articles: the slug is the source path (e.g. `source.html`).
- Series articles: the slug is the series name followed by the source path (e.g. `series-name/source.html`).

### Articles

- `publicationDate` is a required ISO 8601 date.
- `updateDate` is optional. When absent, the article has never been updated.
- `tags` is a list of free-form strings.
- Articles in a series inherit the series-level `tags` in addition to their own.

### Ordering

- The index page shows all articles (both standalone and from series) together, ordered by `publicationDate` descending (newer first), then by `title` ascending.
- Within a series, articles are ordered by their position in the array.

## Consequences
- **Single source of truth**: one file describes everything the generator needs to run.
- **Platform-independent paths**: arrays of segments avoid separator issues across operating systems.
- **Straightforward deserialization**: the structure maps directly to Rust types via serde.
- **Schema is rigid**: changes to the input format require updating both the ADR and the deserialization code.
