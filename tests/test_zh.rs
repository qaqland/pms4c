use pulldown_cmark::{Event, Parser, html};

fn events_to_html<'a>(events: impl IntoIterator<Item = Event<'a>>) -> String {
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, events.into_iter());
    html_buf
}

fn pms4c_assert(input: &str, expected: &str) {
    let i_options = pulldown_cmark::Options::all();
    let i_parser = Parser::new_ext(input, i_options);
    let i_events = pms4c::pms4c(i_parser);
    let i_html = events_to_html(i_events);

    let e_options = pulldown_cmark::Options::all();
    let e_parser = Parser::new_ext(expected, e_options);
    let e_html = events_to_html(e_parser);

    assert_eq!(i_html, e_html);
}

// 1. 纯中文单换行
#[test]
fn test_pure_chinese_single_break() {
    let input = "春天来了，\n花儿开了。";
    let expected = "春天来了，花儿开了。";
    pms4c_assert(input, expected);
}

// 2. 纯中文双换行（应保留段落分隔）
#[test]
fn test_pure_chinese_double_break() {
    let input = "春天来了，\n\n花儿开了。";
    let expected = "春天来了，\n\n花儿开了。";
    pms4c_assert(input, expected);
}

// 3. 中英文混合换行（应保留换行）
#[test]
fn test_mixed_chinese_english_break() {
    let input = "Hello\n世界";
    let expected = "Hello\n世界";
    pms4c_assert(input, expected);
}

// 4. 纯英文换行（应保留换行）
#[test]
fn test_pure_english_break() {
    let input = "Hello\nWorld";
    let expected = "Hello\nWorld";
    pms4c_assert(input, expected);
}

// 5. 数字和中文换行（应保留换行）
#[test]
fn test_number_chinese_break() {
    let input = "123\n中文";
    let expected = "123\n中文";
    pms4c_assert(input, expected);
}

// 6. 中文标点换行（应去除换行）
#[test]
fn test_chinese_punctuation_break() {
    let input = "你好，\n世界！";
    let expected = "你好，世界！";
    pms4c_assert(input, expected);
}

// 7. 空行处理
#[test]
fn test_empty_line() {
    let input = "\n";
    let expected = "\n";
    pms4c_assert(input, expected);
}

// 8. 多个连续换行（应保留）
#[test]
fn test_multiple_consecutive_breaks() {
    let input = "第一行\n\n\n第二行";
    let expected = "第一行\n\n\n第二行";
    pms4c_assert(input, expected);
}

// 9. 无换行文本
#[test]
fn test_no_breaks() {
    let input = "这是一行没有换行的文本。";
    let expected = "这是一行没有换行的文本。";
    pms4c_assert(input, expected);
}

// 10. 长文本处理
#[test]
fn test_long_text() {
    let input = "很长的一段中文文本，有很多换行符\n用于测试工具的处理能力，\n看看它是否能正确去除换行而不影响内容。";
    let expected = "很长的一段中文文本，有很多换行符用于测试工具的处理能力，看看它是否能正确去除换行而不影响内容。";
    pms4c_assert(input, expected);
}

// 11. 开头换行（应保留）
#[test]
fn test_leading_break() {
    let input = "\n开头有换行";
    let expected = "\n开头有换行";
    pms4c_assert(input, expected);
}

// 12. 结尾换行（应保留）
#[test]
fn test_trailing_break() {
    let input = "结尾有换行\n";
    let expected = "结尾有换行\n";
    pms4c_assert(input, expected);
}

// 13. 只有换行符
#[test]
fn test_only_breaks() {
    let input = "\n\n";
    let expected = "\n\n";
    pms4c_assert(input, expected);
}

// 14. 混合语言换行（应保留）
#[test]
fn test_mixed_language_break() {
    let input = "中文English\n混合文本";
    let expected = "中文English\n混合文本";
    pms4c_assert(input, expected);
}

// 15. 特殊字符换行（应保留）
#[test]
fn test_special_characters_break() {
    let input = "中文@#$\n%^&中文";
    let expected = "中文@#$\n%^&中文";
    pms4c_assert(input, expected);
}

// 16. Unicode中文字符换行（应去除）
#[test]
fn test_unicode_chinese_break() {
    let input = "中文\u{4e00}\n\u{4e01}中文";
    let expected = "中文\u{4e00}\u{4e01}中文";
    pms4c_assert(input, expected);
}

// 17. 编码处理（UTF-8）
#[test]
fn test_utf8_encoding() {
    let input = "中文测试\n编码处理";
    let expected = "中文测试编码处理";
    pms4c_assert(input, expected);
}

// 18. 错误输入处理
#[test]
fn test_non_string_input() {
    // 假设你的函数只接受字符串，这个测试可能需要调整
    // 这里演示如何处理空输入
    let input = "";
    let expected = "";
    pms4c_assert(input, expected);
}

// 额外测试：中文之间包含英文单词
#[test]
fn test_chinese_with_embedded_english() {
    let input = "这是一个包含\n英文单词的句子";
    let expected = "这是一个包含英文单词的句子";
    pms4c_assert(input, expected);
}

// 额外测试：换行在标点符号后
#[test]
fn test_break_after_punctuation() {
    let input = "第一句。\n第二句";
    let expected = "第一句。第二句";
    pms4c_assert(input, expected);
}

// 额外测试：复杂混合场景
#[test]
fn test_complex_mixed_scenario() {
    let input = "标题\n\n正文第一段，\n包含中文和English。\n\n第二段123\n数字和中文。";
    let expected = "标题\n\n正文第一段，包含中文和English。\n\n第二段123\n数字和中文。";
    pms4c_assert(input, expected);
}
