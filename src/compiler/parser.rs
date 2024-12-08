use super::tokenizer::{self, names, token};
use crate::nodes::{
    node, types::binops, types::expresions, types::node_type, types::statement, types::unops,
};

/*
* Interpret tokens, following the EBNF (See EBNF.md), into nodes
* for the AST
*
* First step of the parsing process
*/
#[derive(Debug,Clone)]
pub struct AST_parser {
    tokens: Vec<tokenizer::names>,
    AST: node,
}

#[derive(Debug)]
pub enum AST_Errors {
    sintaxis(String, String),
}

impl AST_parser {
    pub fn new_from(tokens_origin: Vec<tokenizer::names>) -> Self {
        let mut tokens = tokens_origin;
        tokens.reverse();
        return Self {
            tokens,
            AST: node {
                type_node: Box::new(node_type::block(Vec::new())),
            },
        };
    }


    fn peek(self) -> Option<tokenizer::names> {
        return self.tokens.last().cloned();
    }

    //To optimize
    fn parse_variable(mut self, token: &Vec<u8>) -> node {
        match String::from_utf8_lossy(&token).to_string().as_str() {
            "return" => {
                return node{ type_node: Box::new(node_type::return_node(self.clone().parse_expression(self.clone().tokens.pop().expect("EOF")))) };
            },
            "if" => {
                let statement = self.clone().tokens.pop().expect("EOF"); 
                self.clone().tokens.pop().expect("EOF");
                return node{type_node: Box::new(node_type::statement(statement::if_node(self.clone().parse_expression(statement), self.clone().parse_block())))};
            },
            "elif" => {
                let statement = self.clone().tokens.pop().expect("EOF"); 
                self.clone().tokens.pop().expect("EOF");
                return node{type_node: Box::new(node_type::statement(statement::elif_node(self.clone().parse_expression(statement), self.clone().parse_block())))};
            },
            "else" => {
                self.clone().tokens.pop().expect("EOF"); 
                return node{type_node: Box::new(node_type::statement(statement::else_node(self.clone().parse_block())))};
            },
            a => return node{type_node: Box::new(node_type::variable(token.to_vec()))}
        }
    }

    pub fn parse_block(mut self) -> node{
        let mut block = Vec::new();
        dbg!("starting");
        while let Some(value) = self.clone().tokens.pop() {
            dbg!(self.clone().tokens);
            match value {
                tokenizer::names::variable(vec) => {
                    println!("variable");
                    let m = self.clone().parse_variable(&vec);
                    dbg!(&m);
                    block.push(m);
                    dbg!(self.tokens.pop());
                }
                tokenizer::names::literal(vec) => (),

                tokenizer::names::digits(vec) => (),

                tokenizer::names::EOF => (),
                tokenizer::names::operation(vec) => (),

                tokenizer::names::left_bracket => {
                    self.clone().tokens.pop().expect("EOF");
                    block.push(self.clone().parse_block());
                },

                tokenizer::names::right_bracket => return node{type_node: Box::new(node_type::block(block))},
            };
        }
        return node{type_node: Box::new(node_type::block(block))};
    }

    fn parse_expression(self, token: tokenizer::names) -> node {
        let current_node: node = match token {
            names::variable(vec) => node{type_node: Box::new(node_type::variable(vec))},
            names::literal(vec) => node { type_node: Box::new(node_type::expression(expresions::literal(vec))) },
            names::digits(vec) => node { type_node: Box::new(node_type::expression(expresions::digits(vec))) },
            names::EOF => panic!("EOF"),
            _ => panic!("Wrong expression"),
        };

        let binding = self.clone();
        let next_token = binding.tokens.last().expect("EOF");
        match next_token {
            names::operation(a) => { return self.clone().parse_operation(self.clone().tokens.pop().expect("EOF").value(),current_node)},
            _ => (),
        }
        return current_node;
    }

    fn parse_operation(self, token: Vec<u8>,current: node) -> node{
        match String::from_utf8_lossy(&token).to_string().as_str() {
            "+" => return node{type_node: Box::new(node_type::expression(expresions::binop(current, binops::plus, self.clone().parse_expression(self.clone().tokens.pop().expect("EOF")))))},
            _ => panic!("todo"),
        }
    }
}
