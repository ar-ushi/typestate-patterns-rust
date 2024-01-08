mod textformating;

fn main() {
    let raw: textformating::RawText = textformating::RawText::new("testing the formatter");
    let parsed: textformating::ParsedText = raw.parse();
    let formatted : textformating::FormattedText = parsed.format();
    print!("{:?}", formatted)
}
