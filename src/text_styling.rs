//this module sets styles for text such as bold, lines, colors, etc

pub enum StyleCode {
    Color(ColorCode),
    Style(TextStyle),
}
pub enum ColorCode{
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White
}
pub enum TextStyle {
    Bold,
    Dim,
    Italic,
    Underline,
    Inverse,
    Hidden,
    Strikethrough,
    Overline,
}

pub struct Chalk {
    pub text: String,
    pub style: Vec<StyleCode>
}

pub trait Colors {
    fn new() -> Self;

    fn colors<S: AsRef<str>>(&mut self, text: S, color: StyleCode) -> &mut Self;
    fn black<S: AsRef<str>>(&mut self, text: S) -> &mut Self;

    fn red<S: AsRef<str>>(&mut self, text: S) -> &mut Self;

    fn green<S: AsRef<str>>(&mut self, text: S) -> &mut Self;

    fn yellow<S: AsRef<str>>(&mut self, text: S) -> &mut Self;

    fn blue<S: AsRef<str>>(&mut self, text: S) -> &mut Self;

    fn magenta<S: AsRef<str>>(&mut self, text: S) -> &mut Self;

    fn cyan<S: AsRef<str>>(&mut self, text: S) -> &mut Self;

    fn white<S: AsRef<str>>(&mut self, text: S) -> &mut Self;
}

pub trait Styles {
    fn bold(&mut self) -> &mut Self;
    fn dim(&mut self) -> &mut Self;
    fn italic(&mut self) -> &mut Self;
    fn underline(&mut self) -> &mut Self;
    fn hidden(&mut self) -> &mut Self;
    fn strikethrough(&mut self) -> &mut Self;
    fn overline(&mut self) -> &mut Self;
}

impl ColorCode{
    pub fn to_code(&self) -> u8{
        match self {
            ColorCode::Black => 30,
            ColorCode::Red => 31,
            ColorCode::Green => 32,
            ColorCode::Yellow => 33,
            ColorCode::Blue => 34,
            ColorCode::Magenta => 35,
            ColorCode::Cyan => 36,
            ColorCode::White => 37,
        }
    }
}

impl TextStyle {
    pub fn to_code(&self) -> u8 {
        match self {
            TextStyle::Bold => 1,
            TextStyle::Dim => 2,
            TextStyle::Italic => 3,
            TextStyle::Underline => 4,
            TextStyle::Inverse => 7,
            TextStyle::Hidden => 8,
            TextStyle::Strikethrough => 9,
            TextStyle::Overline => 53,
        }
    }
}

impl StyleCode {
    pub fn to_code(&self) -> String {
        match self {
            StyleCode::Color(c) => c.to_code().to_string(),
            StyleCode::Style(s) => s.to_code().to_string(),
        }
    }
}

impl Colors for Chalk {

    fn new() -> Self {
        Chalk {
            text: String::new(),
            style: Vec::new(),
        }
    }

    fn colors<S: AsRef<str>>(&mut self, text: S, mut color: StyleCode) -> &mut Self{
        self.text = format!("\x1b[{}m{}\x1b[0m", color.to_code(), text.as_ref());
        self
    }

    fn black<S: AsRef<str>>(&mut self, text: S) -> &mut Self{
        self.colors(text, StyleCode::Color(ColorCode::Black));

        self
    }

    fn red<S: AsRef<str>>(&mut self, text: S) -> &mut Self{
        self.colors(text, StyleCode::Color(ColorCode::Red));
        self
    }

    fn green<S: AsRef<str>>(&mut self, text: S) -> &mut Self{
        self.colors(text, StyleCode::Color(ColorCode::Green));
        self
    }
    fn yellow<S: AsRef<str>>(&mut self, text: S) -> &mut Self{
        self.colors(text, StyleCode::Color(ColorCode::Yellow));
        self
    }

    fn blue<S: AsRef<str>>(&mut self, text: S) -> &mut Self{
        self.colors(text, StyleCode::Color(ColorCode::Blue));
        self
    }

    fn magenta<S: AsRef<str>>(&mut self, text: S) -> &mut Self{
        self.colors(text, StyleCode::Color(ColorCode::Magenta));
        self
    }

    fn cyan<S: AsRef<str>>(&mut self, text: S) -> &mut Self{
        self.colors(text, StyleCode::Color(ColorCode::Cyan));
        self
    }

    fn white<S: AsRef<str>>(&mut self, text: S) -> &mut Self{
        self.colors(text, StyleCode::Color(ColorCode::White));
        self
    }

}


impl Styles for Chalk {
    fn bold(&mut self) -> &mut Self {
        self.style.push(StyleCode::Style(TextStyle::Bold));
        self
    }

    fn dim(&mut self) -> &mut Self {
        self.style.push(StyleCode::Style(TextStyle::Dim));
        self
    }

    fn italic(&mut self) -> &mut Self {
        self.style.push(StyleCode::Style(TextStyle::Italic));
        self
    }
    fn underline(&mut self) -> &mut Self {
        self.style.push(StyleCode::Style(TextStyle::Underline));
        self
    }

    fn hidden(&mut self) -> &mut Self {
        self.style.push(StyleCode::Style(TextStyle::Inverse));
        self
    }

    fn strikethrough(&mut self) -> &mut Self {
        self.style.push(StyleCode::Style(TextStyle::Overline));
        self
    }
    fn overline(&mut self) -> &mut Self {
        self.style.push(StyleCode::Style(TextStyle::Overline));
        self
    }
}



impl Chalk {
    pub fn display(&self) {
        let codes = self.style.iter()
            .map(|style| match style {
                StyleCode::Color(color) => color.to_code(),
                StyleCode::Style(style) => style.to_code(),
            })
            .map(|code| code.to_string())
            .collect::<Vec<_>>()
            .join(";");

        println!("\x1b[{}m{}\x1b[0m", codes, self.text);
    }
}