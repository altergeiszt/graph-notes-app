use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ParsedNote {
    pub title : String,
    pub frontmatter: HashMap<String, serde_json::Value>,
    pub body: String,
    pub tags_inline: Vec<String>,
    pub tags_frontmatter: Vec<String>,
}

pub fn parse_note(path: &Path, content: &str) -> ParsedNote {
    let (frontmatter_raw,body) = split_frontmatter(content);
    let frontmatter = parse_frontmatter_yaml(frontmatter_raw);

    let title = extract_title(&frontmatter, body, path);
    let tags_frontmatter = extract_frontmatter_tags(&frontmatter);
    let tags_inline = extract_inline_tags(body);

    ParedNote {title, frontmatter, body
    , tags_inline, tags_frontmatter};

    /// Splits the content into frontmatter and body. If no frontmatter is found, returns an empty string for frontmatter.
    fn split_frontmatter(content: &str) -> (&str, &str) {
        if !content.starts_with("---") {
            return ("", content);
        }
        let after_open = &content[3..];
        let newline_pos = after_open.find('\n').map(|pos| pos + 1).unwrap_or(0);
        let search_region = &after_open[newline_pos..];
        if let Some(close) = search_region.find("\n---") {
            let fm_end = 3 + newline_pos + close;
            let body_start = fm_end + 4;
            let body_start = if content.as_bytes().get(body_start) == Some(&b'\n') {
                body_start + 1
            } else {
                body_start
            };
            (&content[3..fm_end], &content[body_start..])
        } else {
            ("", content)
        }
    }
}