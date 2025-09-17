/// 见 https://en.wikipedia.org/wiki/CJK_Unified_Ideographs
const CJK_RANGES: &[(u32, u32)] = &[
    (0x2e80, 0x2eff), // CJK Radicals Supplement
    (0x2f00, 0x2fdf), // Kangxi Radicals
    (0x2ff0, 0x2fff), // Ideographic Description Characters
    (0x3000, 0x303f), // CJK Symbols and Punctuation
    (0x31c0, 0x31ef), // CJK Strokes
    (0x3200, 0x32ff), // Enclosed CJK Letters and Months
    (0x4e00, 0x9fff), // CJK Unified Ideographs
    (0x3400, 0x4dbf), // CJK Unified Ideographs Extension A
                      // TO BE CONTINUE
];

/// 检查字符是否为 CJK 字符
pub fn is_cjk_character(c: char) -> bool {
    let code = c as u32;
    CJK_RANGES
        .iter()
        .any(|&(start, end)| code >= start && code <= end)
}

/// 检查字符串的第一个字符是否为 CJK 字符
pub fn starts_with_cjk(s: &str) -> bool {
    s.chars().next().map_or(false, is_cjk_character)
}

/// 检查字符串的最后一个字符是否为 CJK 字符
pub fn ends_with_cjk(s: &str) -> bool {
    s.chars().last().map_or(false, is_cjk_character)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_cjk_character() {
        assert!(is_cjk_character('中'));
        assert!(is_cjk_character('文'));
        assert!(is_cjk_character('日'));
        assert!(is_cjk_character('本'));
        // assert!(is_cjk_character('한'));
        // assert!(is_cjk_character('국'));

        assert!(!is_cjk_character('A'));
        assert!(!is_cjk_character('1'));
        assert!(!is_cjk_character('!'));
        assert!(!is_cjk_character(' '));
        assert!(!is_cjk_character('á'));
    }

    #[test]
    fn test_starts_with_cjk() {
        assert!(starts_with_cjk("中文测试"));
        assert!(starts_with_cjk("日本語"));
        // assert!(starts_with_cjk("한국어"));
        assert!(starts_with_cjk("中A混合"));

        assert!(!starts_with_cjk("ABC中文"));
        assert!(!starts_with_cjk("123中文"));
        assert!(!starts_with_cjk("!@中文"));
        assert!(!starts_with_cjk(" abc"));

        assert!(!starts_with_cjk(""));
    }

    #[test]
    fn test_ends_with_cjk() {
        assert!(ends_with_cjk("测试中文"));
        assert!(ends_with_cjk("日本語"));
        assert!(ends_with_cjk("English中"));
        assert!(ends_with_cjk("123文"));

        assert!(!ends_with_cjk("中文ABC"));
        assert!(!ends_with_cjk("中文123"));
        assert!(!ends_with_cjk("中文!@"));
        assert!(!ends_with_cjk("abc "));

        assert!(!ends_with_cjk(""));
    }

    #[test]
    fn test_edge_cases() {
        assert!(starts_with_cjk("中"));
        assert!(ends_with_cjk("中"));

        assert!(starts_with_cjk("中文ABC"));
        assert!(!ends_with_cjk("中文ABC"));
        assert!(!starts_with_cjk("ABC中文"));
        assert!(ends_with_cjk("ABC中文"));

        assert!(starts_with_cjk("中文日"));
        assert!(ends_with_cjk("中文日"));
    }
}
