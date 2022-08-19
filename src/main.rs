pub mod ast;
pub mod lexer;

use ast::create_ast;
use lexer::tokenize;

fn main() {
    dbg!(create_ast(tokenize("a = 1 + (2 + 3) * 4 + 5")));
}
