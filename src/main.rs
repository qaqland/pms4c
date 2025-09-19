use indoc::indoc;
use pulldown_cmark_to_cmark::cmark;

fn main() {
    let input = indoc! {"
        那是一个深邃而宁静的秋日黄昏，
        夕阳的余晖。

        > 如同一位技艺精湛的
        > 画家，将漫天云霞渲染成一片绚烂的金红。

        微风轻拂过庭院，惹得那几株老槐树的枝叶沙沙作响，仿佛在**低声絮语**
        着一个古老而神秘的传说。
        *我*独自坐在窗前的书桌旁，面前摊开着一本厚重的、略显古旧的书籍；  
        书页已然泛黄，却依然散发出一种混合着墨香与时光气息的独特味道。
    "};
    let mut buf = String::with_capacity(input.len());
    let options = pulldown_cmark::Options::all();
    let parser = pulldown_cmark::Parser::new_ext(input, options);
    let events = pms4c::pms4c(parser);

    cmark(events.into_iter(), &mut buf).unwrap();
    println!("{}", buf);
}
