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
    fn parse_variable(mut self, token: &Vec<u8>) -> (node, Self) {
        match String::from_utf8_lossy(&token).to_string().as_str() {
            "return" => {
                let exp = self.tokens.pop().expect("EOF");
                let bin = self.clone().parse_expression();
                self = bin.1;
                return (node{ type_node: Box::new(node_type::return_node(bin.0)) }, self);
            },
            "if" => {
                println!("OT");
                let statement = self.tokens.pop().expect("EOF"); 
                let exp = self.clone().parse_expression();
                self = exp.1;
                let block =  self.clone().parse_block(); 
                self = block.1;
                return (node{type_node: Box::new(node_type::statement(statement::if_node(exp.0, block.0)))}, self);
            },
            "elif" => {
                let statement = self.clone().tokens.pop().expect("EOF"); 
                self.clone().tokens.pop().expect("EOF");
                let exp = self.clone().parse_expression();
                self = exp.1;
                let block = self.clone().parse_block();
                self =  block.1;
                return (node{type_node: Box::new(node_type::statement(statement::elif_node(exp.0, block.0)))}, self);
            },
            "else" => {
                self.clone().tokens.pop().expect("EOF"); 
                let block = self.clone().parse_block();
                self = block.1;
                return (node{type_node: Box::new(node_type::statement(statement::else_node(block.0)))}, self);
            },
            a => return (node{type_node: Box::new(node_type::variable(token.to_vec()))}, self)
        }
    }

    pub fn parse_block(mut self) -> (node,Self){
        let mut block = Vec::new();
        dbg!("starting");
        while let Some(value) = self.clone().tokens.pop() {
            match value {
                tokenizer::names::variable(vec) => {
                    let m = self.clone().parse_variable(&vec);
                    self = m.1;
                    block.push(m.0);
                    self.tokens.pop();
                }
                tokenizer::names::literal(vec) => (),

                tokenizer::names::digits(vec) => (),

                tokenizer::names::EOF => (),
                tokenizer::names::operation(vec) => (),

                tokenizer::names::left_bracket => {
                    self.tokens.pop().expect("EOF");
                    let bin = self.clone().parse_block();
                    self = bin.1;
                    block.push(bin.0);
                },

                tokenizer::names::right_bracket => {
                    return (node{type_node: Box::new(node_type::block(block))},self);
                },
            };
        }
        return (node{type_node: Box::new(node_type::block(block))},self);
    }

    fn parse_expression(mut self) -> (node, Self) {
        let token = self.tokens.pop().expect("EOF");
        dbg!(&token);
        let current_node: node = match token {
            names::variable(vec) => node{type_node: Box::new(node_type::variable(vec))},
            names::literal(vec) => node { type_node: Box::new(node_type::expression(expresions::literal(vec))) },
            names::digits(vec) => node { type_node: Box::new(node_type::expression(expresions::digits(vec))) },
            names::EOF => panic!("EOF"),
            a => panic!("Wrong expression {}",String::from_utf8_lossy(&a.value()).to_string()),
        };

        let next_token = match self.tokens.last() {
            Some(a) => a,
            None => return (current_node,self),
        };
        match next_token {
            names::operation(a) => { 
                println!("Next is operator");
                let bin = self.clone().parse_operation(current_node);
                self = bin.1;
                return (bin.0,self);},
            _ => (),
        }
        return (current_node,self);
    }

    fn parse_operation(mut self,current: node) -> (node,Self){
        let token = self.tokens.pop().expect("EOF").value();
        println!("{}", String::from_utf8_lossy(&token).to_string());
        match String::from_utf8_lossy(&token).to_string().as_str() {
            "+" => {
                println!("plus");
                //self.tokens.pop();
                let bin = self.clone().parse_expression();
                self = bin.1;
                return (node{type_node: Box::new(node_type::expression(expresions::binop(current, binops::plus, bin.0)))},self);},
            _ => panic!("todo"),
        }
    }


}
