use crate::lexer::{Op, Operator, Token};
use std::iter::Peekable;

#[derive(Debug)]
pub enum AST {
    Id(String),
    Lit(f64),
    BinOp {
        op: Op,
        left: Box<AST>,
        right: Box<AST>,
    },
    UnOp {
        op: Op,
        right: Box<AST>,
    },
    IfClause {
        condition: Box<AST>,
        if_body: Box<AST>,
        else_body: Option<Box<AST>>,
    },
    Function {
        id: Box<AST>,
        body: Box<AST>,
    },
    Call {
        id: Box<AST>,
        args: Vec<Box<AST>>,
    },
}

fn parse_next_operand(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> AST {
    if let Some(token) = tokens.next() {
        use Token::*;
        match token {
            Id(id) => AST::Id(id),
            Lit(lit) => AST::Lit(lit),
            LeftBracket => parse_bin_op(parse_next_operand(tokens), tokens),
            _ => panic!("Invalid operand!"),
        }
    } else {
        panic!("No token found, EOI")
    }
}

fn parse_bin_op(left: AST, tokens: &mut Peekable<impl Iterator<Item = Token>>) -> AST {
    let op;
    let next_op;
    let ast: AST;

    if let Some(Token::Op(x)) = tokens.next() {
        op = x;
    } else {
        panic!("Next token is not an op!")
    }

    let mut right = parse_next_operand(tokens);

    match tokens.peek() {
        Some(Token::Op(x)) => {
            // Next op exists.
            next_op = x;

            if op.priority() < next_op.priority() {
                // Next op's priority is higher. Build AST of next op and use it as right operand.
                right = parse_bin_op(right, tokens);
            }

            ast = AST::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Some(Token::RightBracket) => {
            tokens.next();
            ast = AST::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
            return ast;
        }
        _ => {
            // No next op. It's EOI.
            ast = AST::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
            return ast;
        }
    }
    if let Some(_token) = tokens.peek() {
        if *_token != Token::RightBracket {
            return parse_bin_op(ast, tokens);
        } else {
            tokens.next();
        }
    } else {
    }
    ast
}

pub fn create_ast(tokens: Vec<Token>) -> AST {
    let tokens = &mut tokens.into_iter().peekable();
    parse_bin_op(parse_next_operand(tokens), tokens)
}
