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
#[derive(Debug, Clone)] 
pub struct WikilinkMatch { 
    pub target: String,          // Note title or path (before #) 
    pub alias: Option<String>,   // Display text after | 
    pub section_anchor: Option<String>,  // After # (heading) 
    pub block_id: Option<String>,        // After #^ (block ref) 
    pub is_transclusion: bool,   // True if preceded by ! 
    pub line_number: u32, 
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

pub fn extract_wikilinks(content: &str) -> Vec<WikilinkMatch> { 
    // Match: optional ! prefix, [[, target, optional |alias or #anchor, ]] 
    // Regex breakdown: 
    //  (!?)       — capture optional transclusion prefix 
    //  \[\[       — opening brackets 
    //  ([^\]#|]+) — target name (no ], #, |) 
    //  (#\^?([^\]|]+))? — optional #section or #^blockid 
    //  (\|([^\]]+))? — optional |alias 
    //  \]\]       — closing brackets 
    let re = regex::Regex::new( 
        r"(?m)(!?)\[\[([^\]#|]+?)(?:#(\^?)([^\]|]+?))?(?:\|([^\]]*))??\]\]"
    ).unwrap(); 
     let mut results = Vec::new(); 
    let mut in_code_block = false; 
     for (line_no, line) in content.lines().enumerate() { 
        // Skip content inside fenced code blocks 
        if line.trim_start().starts_with("```") { 
            in_code_block = !in_code_block; 
            continue; 
        } 
        if in_code_block { continue; } 
         for cap in re.captures_iter(line) { 
            let is_transclusion = &cap[1] == "!"; 
            let target = cap[2].trim().to_string(); 
            let block_prefix = cap.get(3).map(|m| m.as_str()).unwrap_or(""); 
            let anchor_or_block = cap.get(4).map(|m| m.as_str().to_string()); 
            let alias = cap.get(5)
                .map(|m| m.as_str().to_string())
                .filter(|s| !s.is_empty());
             let (section_anchor, block_id) = if block_prefix == "^" { 
                (None, anchor_or_block) 
            } else { 
                (anchor_or_block, None)             }; 
             results.push(WikilinkMatch { 
                target, 
                alias, 
                section_anchor, 
                block_id, 
                is_transclusion, 
                line_number: (line_no + 1) as u32, 
            }); 
        } 
    } 
    results 
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
        (&content[3 + newline_pos..fm_end], &content[body_start..])
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    // ── split_frontmatter ────────────────────────────────────────────

    #[test]
    fn test_no_frontmatter_returns_full_content_as_body() {
        let (fm, body) = split_frontmatter("# Hello\n\nJust a note.");
        assert_eq!(fm, "");
        assert_eq!(body, "# Hello\n\nJust a note.");
    }

    #[test]
    fn test_valid_frontmatter_is_split_correctly() {
        let content = "---\ntitle: My Note\n---\nBody here.";
        let (fm, body) = split_frontmatter(content);
        assert_eq!(fm, "title: My Note");
        assert_eq!(body, "Body here.");
    }

    #[test]
    fn test_only_frontmatter_no_body() {
        // Edge case from §3.1.3: body = empty string
        let content = "---\ntitle: Only FM\n---\n";
        let (_, body) = split_frontmatter(content);
        assert_eq!(body, "");
    }

    #[test]
    fn test_unclosed_frontmatter_treated_as_no_frontmatter() {
        // No closing ---, whole content becomes body
        let content = "---\ntitle: Missing Close\nBody leaks out.";
        let (fm, body) = split_frontmatter(content);
        assert_eq!(fm, "");
        assert_eq!(body, content);
    }

    // ── extract_title priority chain ─────────────────────────────────

    #[test]
    fn test_title_from_frontmatter_wins_over_h1() {
        let path = Path::new("notes/my-note.md");
        let content = "---\ntitle: FM Title\n---\n# H1 Title";
        let result = parse_note(path, content);
        assert_eq!(result.title, "FM Title");
    }

    #[test]
    fn test_title_falls_back_to_h1_when_no_frontmatter_title() {
        let path = Path::new("notes/my-note.md");
        let content = "---\ndate: 2026-06-01\n---\n# The Real Title";
        let result = parse_note(path, content);
        assert_eq!(result.title, "The Real Title");
    }

    #[test]
    fn test_title_falls_back_to_filename_when_no_frontmatter_or_h1() {
        let path = Path::new("notes/my-note.md");
        let content = "Just some text, no heading.";
        let result = parse_note(path, content);
        assert_eq!(result.title, "my-note");
    }

    // ── frontmatter YAML edge cases ──────────────────────────────────

    #[test]
    fn test_malformed_yaml_does_not_crash_returns_empty_map() {
        // Edge case from §3.1.3: unwrap_or_default() on serde_yaml
        let path = Path::new("notes/bad.md");
        let content = "---\n: this is not valid yaml: [\n---\nBody.";
        let result = parse_note(path, content);
        assert!(result.frontmatter.is_empty());
        assert_eq!(result.body.trim(), "Body.");
    }

    #[test]
    fn test_tags_as_array_parsed_correctly() {
        let path = Path::new("notes/tagged.md");
        let content = "---\ntags: [rust, programming]\n---\nBody.";
        let result = parse_note(path, content);
        assert!(result.tags_frontmatter.contains(&"rust".to_string()));
        assert!(result.tags_frontmatter.contains(&"programming".to_string()));
    }

    // ── inline tag extraction ────────────────────────────────────────

    #[test]
    fn test_inline_tags_extracted_from_body() {
        let path = Path::new("notes/tags.md");
        let content = "This note is about #rust and #programming/systems.";
        let result = parse_note(path, content);
        assert!(result.tags_inline.contains(&"rust".to_string()));
        assert!(result.tags_inline.contains(&"programming/systems".to_string()));
    }

    // ── extract_wikilinks ────────────────────────────────────────────

    #[test]
    fn test_basic_wikilink() {
        let links = extract_wikilinks("See [[Note Name]] for details.");
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "Note Name");
        assert!(links[0].alias.is_none());
        assert!(links[0].section_anchor.is_none());
    }

    #[test]
    fn test_wikilink_with_alias() {
        let links = extract_wikilinks("Click [[Target Note|here]] to read.");
        assert_eq!(links[0].target, "Target Note");
        assert_eq!(links[0].alias.as_deref(), Some("here"));
    }

    #[test]
    fn test_wikilink_with_section_anchor() {
        let links = extract_wikilinks("See [[Note Name#Section Heading]].");
        assert_eq!(links[0].target, "Note Name");
        assert_eq!(links[0].section_anchor.as_deref(), Some("Section Heading"));
        assert!(links[0].block_id.is_none());
    }

    #[test]
    fn test_wikilink_with_block_id() {
        let links = extract_wikilinks("Ref [[Note Name#^block-123]].");
        assert_eq!(links[0].target, "Note Name");
        assert_eq!(links[0].block_id.as_deref(), Some("block-123"));
        assert!(links[0].section_anchor.is_none());
    }

    #[test]
    fn test_transclusion_flag() {
        let links = extract_wikilinks("![[Embedded Note]]");
        assert!(links[0].is_transclusion);
        assert_eq!(links[0].target, "Embedded Note");
    }

    #[test]
    fn test_wikilink_inside_fenced_code_block_is_skipped() {
        // Edge case from §5.1.3: skipped inside ```
        let content = "```\n[[Should Not Match]]\n```";
        let links = extract_wikilinks(content);
        assert!(links.is_empty());
    }

    #[test]
    fn test_empty_target_is_skipped() {
        // Edge case: [[]] should not match
        let links = extract_wikilinks("[[]]");
        assert!(links.is_empty());
    }

    #[test]
    fn test_wikilink_with_spaces_in_target() {
        let links = extract_wikilinks("[[Note with spaces]]");
        assert_eq!(links[0].target, "Note with spaces");
    }

    #[test]
    fn test_empty_alias_treated_as_none() {
        let links = extract_wikilinks("[[Target|]]");
        assert!(links[0].alias.is_none());
    }

    #[test]
    fn test_line_number_is_recorded_correctly() {
        let content = "Line 1\nLine 2\n[[My Note]]\nLine 4";
        let links = extract_wikilinks(content);
        assert_eq!(links[0].line_number, 3);
    }

    #[test]
    fn test_multiple_wikilinks_in_one_note() {
        let content = "See [[Alpha]] and [[Beta]] and [[Gamma]].";
        let links = extract_wikilinks(content);
        assert_eq!(links.len(), 3);
        assert_eq!(links[0].target, "Alpha");
        assert_eq!(links[1].target, "Beta");
        assert_eq!(links[2].target, "Gamma");
    }
}

