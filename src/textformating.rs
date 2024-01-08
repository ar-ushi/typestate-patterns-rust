type Word = &'static str;

pub struct RawText(&'static str);
#[derive(Debug)]
pub struct ParsedText(Vec<Word>);
#[derive(Debug)]
pub struct FormattedText(String);

//Expected Transitions
// RawText --> ParsedText
// ParsedText --> FormattedText

impl RawText{
    pub fn new(raw: &'static str) -> Self{
        Self(raw)
    }

    pub fn parse(self) -> ParsedText{
      let parsed = self.0.split(' ').collect();
      ParsedText(parsed)
    }
}

impl ParsedText{
    pub fn format(self) -> FormattedText{
        FormattedText(self.0.join("0"))
    }
}

impl FormattedText{
    pub fn as_str(&self) -> &str{
        self.0.as_str()
    }
}