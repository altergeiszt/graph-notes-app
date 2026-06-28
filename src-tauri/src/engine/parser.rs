use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct ParsedNote {
    pub title: String,
    pub frontmatter: HashMap<String, serde_json::Value>,
    pub body: String,
    pub tags_inline: Vec<String>,
    pub tags_frontmatter: Vec<String>,
}

pub fn parse_note(path: &Path, content: &str) -> ParsedNote {
    let (frontmatter_raw, body) = split_frontmatter(content);
    let frontmatter = parse_frontmatter_yaml(frontmatter_raw);
    let title = extract_title(&frontmatter, body, path);
    let tags_frontmatter = extract_frontmatter_tags(&frontmatter);
    let tags_inline = extract_inline_tags(body);

    ParsedNote {
        title,
        frontmatter,
        body: body.to_string(),
        tags_inline,
        tags_frontmatter,
    }
}

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

fn parse_frontmatter_yaml(yaml: &str) -> HashMap<String, serde_json::Value> {
    serde_yaml::from_str(yaml).unwrap_or_default()
}

fn extract_title(
    frontmatter: &HashMap<String, serde_json::Value>,
    body: &str,
    path: &Path,
) -> String {
    if let Some(serde_json::Value::String(title)) = frontmatter.get("title") {
        if !title.is_empty() {
            return title.clone();
        }
    }
    for line in body.lines() {
        if let Some(title) = line.strip_prefix("# ") {
            let t = title.trim().to_string();
            if !t.is_empty() {
                return t;
            }
        }
    }
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Untitled")
        .to_string()
}

fn extract_frontmatter_tags(frontmatter: &HashMap<String, serde_json::Value>) -> Vec<String> {
    match frontmatter.get("tags") {
        Some(serde_json::Value::Array(arr)) => arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.trim().to_string()))
            .filter(|s| !s.is_empty())
            .collect(),
        Some(serde_json::Value::String(s)) => s
            .split(',')
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect(),
        _ => vec![],
    }
}

fn extract_inline_tags(body: &str) -> Vec<String> {
    let re = Regex::new(r"(?m)(?:^|\s)#([\w/]+)").expect("valid regex");
    re.captures_iter(body)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect()
}
