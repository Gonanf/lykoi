pub mod tokenizer;
use tokenizer::{names, token};
pub mod parser;
pub mod interpreter;
pub mod codegen;
use parser::AST_parser;

use crate::nodes;

pub fn get_types() -> Vec<token> {
    //DEBUG ONLY
    let mut buffer = String::from("a = 10 if a + 2 == 15 { b = 30 } else { b = 20 } b");
    //std::io::stdin().read_line(&mut buffer);
    /////////////

    let mut tokens: Vec<tokenizer::token> = Vec::new();
    for i in buffer.chars() {
        tokens.push(match (tokenizer::token::search_token(i as u8)) {
            Some(a) => a,
            None => continue,
        });
    }

    //DEBUG ONLY
    dbg!(&tokens);
    ////////////
    return tokens;
}

pub fn agroup(tokens: Vec<token>) -> Vec<names> {
    let data = names::agroup_tokens(tokens);
    for tk in &data {
        match tk {
            names::variable(a, ..) => {
                println!("variable(\n{}\n)", String::from_utf8_lossy(&a))
            }
            names::literal(a, ..) => {
                println!("literal(\n{}\n)", String::from_utf8_lossy(&a))
            }
            names::digits(a, ..) => {
                println!("digits(\n{}\n)", String::from_utf8_lossy(&a))
            }
            names::EOF(..) => println!("EOF"),
            names::operation(a, ..) => {
                println!("operation(\n{}\n)", String::from_utf8_lossy(&a))
            }
            names::left_bracket(..) => println!("{{"),
            names::right_bracket(..) => println!("}}"),
            names::left_par(..) => println!("("),
            names::right_par(..) => println!(")"),
        }
    }
    return data;
}

pub fn first_parse(tokens: Vec<names>) -> nodes::node {
    let mut parser = parser::AST_parser::new_from(tokens);
    let ps = parser.parse_block().0;
    dbg!(&ps);
    return ps;
}

pub fn interpretate(tokens: nodes::node) {
    let inter = interpreter::interpreter::new(tokens);
    inter.interpretate();
}
