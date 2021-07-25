use super::lexer::{Lexer, Op, TType, Token};
use crate::{error, Result};
use std::mem::discriminant;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprT {
    String(String),
    Number(f64),
    Var(String),
    Index(String, usize, Option<usize>),
    Operation(Op, Vec<ExprT>),
}
#[derive(Debug, Clone, PartialEq)]
pub enum InstructionT {
    Let(Vec<(String, ExprT)>),
    Print(Vec<ExprT>),
    Loop(
        String, /* Increment */
        f64,    /* Start */
        f64,    /* To */
        f64,    /* Step (default: 1*/
    ),
    Read(Vec<String>),
    Input(Vec<String>),
    Dim(Vec<(String, usize, Option<usize>)>),
    Goto(u32),
    Restore,
    Stop,
    End,
    TTYOut,
    TTYIn,
}
#[derive(Debug, Clone)]
pub struct Instruction {
    line: u32,
    instruction: InstructionT,
}
pub struct Parser {
    input: Vec<Token>,
    output: Vec<Instruction>,
    current: usize,
    file: String,
}
impl Parser {
    pub fn new<T: ToString + Clone>(code: T, file: T) -> Result<Self> {
        Ok(Self {
            input: Lexer::new(code, file.clone()).lex()?.output,
            output: vec![],
            file: file.to_string(),
            current: 0,
        })
    }
    fn peek(&self, offset: usize) -> Option<Token> {
        self.input
            .iter()
            .nth(self.current + offset)
            .and_then(|v| Some(v.clone()))
    }
    fn pop(&mut self) -> Result<Token> {
        self.peek(0).map_or(error!(self.file, self.input.iter().last().map_or(1, |t| t.line) => "Unexpected end of token stream"), |v| {
            self.current += 1;
            Ok(v)
        })
    }
    fn advance(&mut self, ttype: TType) -> Result<Token> {
        if let Some(tok) = self.peek(0) {
            if std::mem::discriminant(&tok.ttype) != std::mem::discriminant(&ttype) {
                error!(self.file, tok.line => "Expected {:?}, found {:?}.", ttype, tok.ttype)
            } else {
                self.pop()
            }
        } else {
            error!(self.file, self.input.iter().last().map_or(1, |t| t.line) => "Unexpected end of token stream")
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }
    fn parse_expr(&mut self) -> Result<ExprT> {
        let tok = self.pop()?;
        match tok.ttype {
            TType::String(s) => Ok(ExprT::String(s)),
            TType::Ident(id)
                if !self.is_at_end() && self.peek(0).unwrap().ttype == TType::OpenParen =>
            {
                self.pop()?;
                let first_idx = self.advance(TType::Number(3.14159265358979323846264338327950288419716939937510582097494459230781640628620899))?;
                if let TType::Number(first_idx) = first_idx.ttype {
                    let first_idx = if first_idx.fract() != 0. {
                        error!(self.file, tok.line => "Invalid index: {}: indices must be valid unsigned integers.", first_idx)
                    } else {
                        Ok(first_idx as usize)
                    }?;
                    let second_idx = if !self.is_at_end()
                        && self.peek(0).unwrap().ttype == TType::Coma
                    {
                        self.pop()?;
                        if let TType::Number(second_idx) = self.advance(TType::Number(2.71828 /* I'm sorry, I don't know many decimals of the Napier constant */))?.ttype {
                            if second_idx.fract() != 0. {
                                error!(self.file, tok.line => "Invalid index: {}: indices must be valid unsigned integers.", second_idx)
                            } else {
                                Ok(Some(second_idx as usize))
                            }?
                        } else {
                            unreachable!();
                        }
                    } else {
                        None
                    };
                    self.advance(TType::CloseParen)?;
                    Ok(ExprT::Index(id.to_string(), first_idx, second_idx))
                } else {
                    unreachable!();
                }
            }
            TType::Ident(s) => Ok(ExprT::Var(s)),
            TType::Number(x) => Ok(ExprT::Number(x)),
            _ => todo!(),
        }
    }
    fn parse_instruction(&mut self) -> Result<InstructionT> {
        let tok = self.pop()?;
        match tok.ttype {
            TType::Let => {
                let mut kv = vec![];
                while !self.is_at_end() && self.peek(0).unwrap().ttype != TType::EOL {
                    let name = self.advance(TType::Ident("".to_string()))?;
                    if let TType::Op(Op::Eq) = self.pop()?.ttype {
                        Ok(())
                    } else {
                        error!(self.file, name.line => "Missing `=` after LET statement.")
                    }?;
                    let value = self.parse_expr()?;
                    if let TType::Ident(id) = name.ttype {
                        kv.push((id, value));
                    } else {
                        unreachable!();
                    }
                }
                Ok(InstructionT::Let(kv))
            }
            _ => unreachable!(),
        }
    }
    fn parse_one(&mut self) -> Result<Instruction> {
        let tok = self.pop()?;
        match tok.ttype {
            TType::LN(n) => Ok(Instruction {
                line: n,
                instruction: self.parse_instruction()?,
            }),
            _ => error!(self.file, tok.line => "Lines must start with a line number."),
        }
    }
}
