use std::collections::HashMap;

pub trait Operator {
    fn priority(&self) -> u8;
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Op {
    Assign,
    Plus,
    Minus,
    Mult,
    Div,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Not,
    And,
    Or,
    Xor,
}

impl Operator for Op {
    fn priority(&self) -> u8 {
        let priorities: HashMap<Op, u8> = HashMap::from([
            (Op::Assign, 0),
            (Op::Plus, 2),
            (Op::Minus, 2),
            (Op::Mult, 3),
            (Op::Div, 3),
            (Op::Eq, 1),
            (Op::Ne, 1),
            (Op::Lt, 1),
            (Op::Gt, 1),
            (Op::Le, 1),
            (Op::Ge, 1),
            (Op::Not, 4),
            (Op::And, 3),
            (Op::Or, 2),
            (Op::Xor, 2),
        ]);
        *priorities.get(self).unwrap()
    }
}

#[derive(Debug, PartialEq)]
pub enum KeyWord {
    Var,
    Def,
    Extern,
    If,
    Then,
    Else,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    KeyWord(KeyWord),
    Id(String),
    Lit(f64),
    Op(Op),
    Comm(String),
    LeftBracket,
    RightBracket,
    Tab,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut output = Vec::<Token>::new();
    let mut input = input.chars().peekable();
    let mut identifier = String::new();

    while let Some(mut cur_chr) = input.next() {
        let mut nxt_chr = *input.peek().unwrap_or(&' ');
        let token = if cur_chr.is_whitespace() {
            if cur_chr == ' ' && nxt_chr == ' ' {
                input.next();
                Token::Tab
            } else {
                continue;
            }
        } else if cur_chr.is_ascii_digit() || cur_chr == '.' {
            while nxt_chr.is_ascii_digit() || nxt_chr == '.' {
                identifier.push(cur_chr);
                cur_chr = input.next().unwrap();
                nxt_chr = *input.peek().unwrap_or(&' ');
            }
            identifier.push(cur_chr);
            Token::Lit(identifier.parse().unwrap())
        } else if cur_chr.is_alphabetic() {
            while nxt_chr.is_alphabetic() {
                identifier.push(cur_chr);
                cur_chr = input.next().unwrap();
                nxt_chr = *input.peek().unwrap_or(&' ');
            }
            identifier.push(cur_chr);

            match identifier.as_str() {
                "var" => Token::KeyWord(KeyWord::Var),
                "def" => Token::KeyWord(KeyWord::Def),
                "extern" => Token::KeyWord(KeyWord::Extern),
                "if" => Token::KeyWord(KeyWord::If),
                "then" => Token::KeyWord(KeyWord::Then),
                "else" => Token::KeyWord(KeyWord::Else),
                _ => Token::Id(identifier.clone()),
            }
        } else if cur_chr == '(' {
            Token::LeftBracket
        } else if cur_chr == ')' {
            Token::RightBracket
        } else if cur_chr.is_ascii_punctuation() {
            while nxt_chr.is_ascii_punctuation() {
                identifier.push(cur_chr);
                cur_chr = input.next().unwrap();
                nxt_chr = *input.peek().unwrap_or(&' ');
            }
            identifier.push(cur_chr);

            match identifier.as_str() {
                "=" => Token::Op(Op::Assign),
                "+" => Token::Op(Op::Plus),
                "-" => Token::Op(Op::Minus),
                "*" => Token::Op(Op::Mult),
                "/" => Token::Op(Op::Div),
                "==" => Token::Op(Op::Eq),
                "!=" => Token::Op(Op::Ne),
                "<" => Token::Op(Op::Lt),
                ">" => Token::Op(Op::Gt),
                "<=" => Token::Op(Op::Le),
                ">=" => Token::Op(Op::Ge),
                "!" => Token::Op(Op::Not),
                "&&" => Token::Op(Op::And),
                "||" => Token::Op(Op::Or),
                "^" => Token::Op(Op::Xor),
                "//" => {
                    let mut comm = String::new();
                    cur_chr = input.next().unwrap_or('\n');
                    while !(cur_chr == '\n' || cur_chr == '\r') {
                        comm.push(cur_chr);
                        cur_chr = input.next().unwrap_or('\n');
                    }
                    comm.push(cur_chr);
                    Token::Comm(comm)
                }
                _ => panic!("Invalid Syntax!"),
            }
        } else {
            panic!("Как мы вообще сюда попали?")
        };
        identifier.clear();
        output.push(token);
    }
    output
}
