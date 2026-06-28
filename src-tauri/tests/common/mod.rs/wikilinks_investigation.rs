#[cfg(test)]
mod wikilink_tests {
    use super::*;

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