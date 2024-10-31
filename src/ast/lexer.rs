#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextSpan {
    start: usize,
    end: usize,
    literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        TextSpan(start, end)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    kind: TokenKind,
    span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self(kind, span)
    }
}

pub struct Lexer<'a> {
    //source: &'a str,
    chars: Chars<'a>,
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars(),
        }
    }
    pub fn next_kind(&mut self) -> Option<Token> {
        while let Some(c) = self.chars.next() {
            match (c) {
                '+' => return TokenKind::Plus,
                _ => {}
            }
        }
        TokenKind::Eof
    }
    pub fn read_next_token(&mut self) -> Token {
        let start = self.offset();
        let kind = self.next_kind();
        let end = self.offset();
        let literal: &str = self.input[start..end].to_string();

        println!("{:?}", literal);

        Token { kind }
    }

    pub fn offset(&self) -> usize {
        self.input.len() - self.chars.as_str().len();
    }

    fn is_number_start(&self) -> bool {}
}
