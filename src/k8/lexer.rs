use crate::{Result, error};

#[derive(Debug, Clone, PartialEq)]
pub enum TType {
    String(String),
    Ident(String),
    Number(f64),
    Eq,
    Neq,
    Ord(bool /* greater */, bool /* equal */),
    Arithmetic(char),
    Let,
    Print,
    OpenParen,
    CloseParen,
}
#[derive(Clone)]
pub struct Token {
    pub ttype: TType,
    pub line: u32,
}

const BUILTINS: [(&str, TType); 2] = [("LET", TType::Let), ("PRINT", TType::Print)];

pub struct Lexer {
    input: String,
    output: Vec<Token>,
    line: u32,
    current: usize,
    file: String,
}
impl Lexer {
    pub fn new(input: impl ToString, file: impl ToString) -> Self {
        Self {
            input: input.to_string().replace("↑", "^"),
            output: vec![],
            line: 1,
            current: 0,
            file: file.to_string(),
        }
    }
    fn finished(&self) -> bool {
        self.current >= self.input.chars().count()
    }
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.current)
    }
    fn advance(&mut self) -> Result<char> {
        if let Some(c) = self.peek() {
            self.current += 1;
            Ok(c)
        } else {
            error!(&self.file, self.line => "Unexpected EOF.")
        }
    }
    fn add_token(&mut self, ttype: TType) {
        self.output.push(Token {
            ttype,
            line: self.line,
        })
    }
    fn number(&mut self, c: char) -> Result<()> {  
        let mut raw = c.to_string ();
        while self.peek().unwrap_or('\0').is_digit(10) {
            raw.push(self.advance()?);
        }
        if self.peek() == Some('.') {
            raw.push(self.advance()?);
        }
        while self.peek().unwrap_or('\0').is_digit(10) {
            raw.push(self.advance()?);
        }
        self.add_token(TType::Number(match raw.parse::<f64>() {
            Ok(x) => x,
            _ => unreachable!(),
        }));

        Ok(())
    }
    fn string(&mut self) -> Result<()> {
        let start = self.current;
        while let Some(c) = self.peek() {
            match c {
                '"' => break,
                '\n' => self.line += 1,
                _ => {}
            }
            self.advance()?;
        }
        if self.finished() {
            error!(self.file, self.line => "Expected `\"`, found EOF.")
        } else {
            self.advance()?; // Closing \n
            self.add_token(TType::String(self.input[start..self.current - 1].to_string()));
            Ok(())
        }
    }
    fn check_many(&self, many: &str) -> bool {
        self.input[self.current - 1..].starts_with(many)
    }
    fn check_builtin(&mut self) -> Option<TType> {
        for (name, builtin) in BUILTINS {
            if self.check_many(name) {
                self.current += name.len() - 1;
                return Some(builtin);
            }
        }
        None
    }
    fn once(& mut self) -> Result<()> {
        let current = self.advance()?;
        match current {
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '(' => self.add_token(TType::OpenParen),
            ')' => self.add_token(TType::CloseParen),
            '"' => return self.string(),
            x if x.is_digit(10) => return self.number(x),
            '+' | '-' | '/' | '*' | '^' => self.add_token(TType::Arithmetic(current)),
            '<' | '>' => {
                let token = if current == '<' && self.peek() == Some('>') {
                    self.advance()?;
                    TType::Neq
                } else {
                    TType::Ord(current == '>', if self.peek() == Some('=') {
                        self.advance()?;
                        true
                    } else { false })
                };
                self.add_token(token);
            },
            '=' => self.add_token(TType::Eq),
            _ => match self.check_builtin() {
                Some(builtin) => self.add_token(builtin),
                None => {
                    let start = self.current - 1;
                    if let Some(c) = self.peek() {
                        if c.is_digit(10) { 
                            self.current += 1;
                        }
                    }
                    self.add_token(TType::Ident(self.input[start..self.current].to_string()))
                }
            },
         }
        Ok(())
    }
    pub fn lex(mut self) -> Result<Self> {
        while !self.finished() {
            self.once()?;
        }
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn numbers() -> Result<()> {
        let tokens = Lexer::new("42 3.1415926", "test")
                        .lex()?
                        .output
                        .into_iter()
                        .map(|t| t.ttype)
                        .collect::<Vec<TType>>();
        assert_eq!(tokens, vec![TType::Number(42.), TType::Number(3.1415926)]);
        Ok(())
    }
    #[test]
    fn ident() -> Result<()> {
        let tokens = Lexer::new("k8 8k KK", "test")
                        .lex()?
                        .output
                        .into_iter()
                        .map(|t| t.ttype)
                        .collect::<Vec<TType>>();
        assert_eq!(tokens, vec![TType::Ident("k8".to_string()), TType::Number(8.), TType::Ident("k".to_string()), TType::Ident("K".to_string()), TType::Ident("K".to_string())]);
        Ok(())
    }
    #[test]
    fn operators() -> Result<()> {
        let tokens = Lexer::new("(5+5*4)=<><=< >=>/↑^-", "test")
                        .lex()?
                        .output
                        .into_iter()
                        .map(|t| t.ttype)
                        .collect::<Vec<TType>>();
        assert_eq!(tokens, vec![TType::OpenParen, TType::Number(5.), TType::Arithmetic('+'), TType::Number(5.), TType::Arithmetic('*'), TType::Number(4.), TType::CloseParen, TType::Eq, TType::Neq, TType::Ord(false, true), TType::Ord(false, false), TType::Ord(true, true), TType::Ord(true, false), TType::Arithmetic('/'), TType::Arithmetic('^'), TType::Arithmetic('^'), TType::Arithmetic('-')]);
        Ok(())
    }
    #[test]
    fn string() -> Result<()> {
        let tokens = Lexer::new("\"Hello, World !\"", "test")
                        .lex()?
                        .output
                        .into_iter()
                        .map(|t| t.ttype)
                        .collect::<Vec<TType>>();
        assert_eq!(tokens, vec![TType::String("Hello, World !".to_string())]);
        Ok(())
    }
    #[test]
    fn builtins() -> Result<()> {
        let tokens = Lexer::new("LETX=5\nPRINT 3", "test")
                        .lex()?
                        .output
                        .into_iter()
                        .map(|t| t.ttype)
                        .collect::<Vec<TType>>();
        assert_eq!(tokens, vec![TType::Let, TType::Ident("X".to_string()), TType::Eq, TType::Number(5.), TType::Print, TType::Number(3.)]);
        Ok(())
    }
}
