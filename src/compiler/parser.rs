use std::f32::consts::E;

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
#[derive(Debug, Clone)]
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
                return (
                    node {
                        type_node: Box::new(node_type::return_node(bin.0)),
                    },
                    self,
                );
            }
            "if" => {
                let statement = self.tokens.pop().expect("EOF");
                let exp = self.clone().parse_expression();
                self = exp.1;
                let block = self.clone().parse_block();
                self = block.1;
                let mut elif_vec = Vec::new();

                while let Some(value) = self.tokens.last() {
                    match String::from_utf8_lossy(&value.clone().value())
                        .to_string()
                        .as_str()
                    {
                        "elif" => {
                            self.tokens.pop();
                            println!("find elif");
                            let val = self.clone().parse_variable(&b"elif".to_vec());
                            self = val.1;
                            elif_vec.push(val.0);
                        }
                        _ => break,
                    }
                }

                let mut else_node = None;
                if let Some(value) = self.tokens.last() {
                    if value.clone().value() == b"else" {
                        self.tokens.pop();
                        let temp_else = self.clone().parse_variable(&b"else".to_vec());
                        self = temp_else.1;
                        else_node = Some(temp_else.0);
                    }
                }
                return (
                    node {
                        type_node: Box::new(node_type::statement(statement::if_node(
                            exp.0, block.0, elif_vec, else_node,
                        ))),
                    },
                    self,
                );
            }
            "elif" => {
                let statement = self.clone().tokens.pop().expect("EOF");
                let exp = self.clone().parse_expression();
                self = exp.1;
                let block = self.clone().parse_block();
                self = block.1;
                return (
                    node {
                        type_node: Box::new(node_type::statement(statement::elif_node(
                            exp.0, block.0,
                        ))),
                    },
                    self,
                );
            }
            "else" => {
                self.tokens.pop().expect("EOF");
                let block = self.clone().parse_block();
                self = block.1;
                return (
                    node {
                        type_node: Box::new(node_type::statement(statement::else_node(block.0))),
                    },
                    self,
                );
            }
            "for" => {
                self.tokens.pop().expect("EOF");
                let exp = self.clone().parse_expression();
                self = exp.1;
                let block = self.clone().parse_block();
                self = block.1;
                return (
                    node {
                        type_node: Box::new(node_type::statement(statement::for_node(
                            exp.0, None,None,block.0
                        ))),
                    },
                    self,
                );
            }

            "while" => {
                self.tokens.pop().expect("EOF");
                let exp = self.clone().parse_expression();
                self = exp.1;
                let block = self.clone().parse_block();
                self = block.1;
                return (
                    node {
                        type_node: Box::new(node_type::statement(statement::while_node(
                            exp.0,block.0
                        ))),
                    },
                    self,
                );
            }

            a => {
                return (
                    node {
                        type_node: Box::new(node_type::variable(token.to_vec())),
                    },
                    self,
                )
            }
        }
    }

    pub fn parse_block(mut self) -> (node, Self) {
        let mut block = Vec::new();
        dbg!("starting");
        while let Some(value) = self.clone().tokens.pop() {
            dbg!(&value);
            match value {
                tokenizer::names::variable(vec) => {
                    let m = self.clone().parse_variable(&vec);
                    self = m.1;
                    block.push(m.0);
                    self.tokens.pop();
                }
                tokenizer::names::literal(vec) | tokenizer::names::digits(vec) => {
                    let m = self.clone().parse_expression();
                    self = m.1;
                    block.push(m.0);
                    self.tokens.pop();
                }

                tokenizer::names::EOF => (),
                tokenizer::names::operation(vec) => {
                    let m = self.clone().parse_expression();
                    self = m.1;
                    block.push(m.0);
                    self.tokens.pop();
                },

                tokenizer::names::left_bracket => {
                    self.tokens.pop().expect("EOF");

                    if block.len() != 0 {
                        let bin = self.clone().parse_block();
                        self = bin.1;
                        block.push(bin.0);
                    }
                }

                tokenizer::names::right_bracket => {
                    self.tokens.pop();
                    return (
                        node {
                            type_node: Box::new(node_type::block(block)),
                        },
                        self,
                    );
                }
            };
        }
        return (
            node {
                type_node: Box::new(node_type::block(block)),
            },
            self,
        );
    }

    fn parse_expression(mut self) -> (node, Self) {
        let token = self.tokens.pop().expect("EOF");
        let current_node: node = match token {
            names::variable(vec) => node {
                type_node: Box::new(node_type::variable(vec)),
            },
            names::literal(vec) => node {
                type_node: Box::new(node_type::expression(expresions::literal(vec))),
            },
            names::digits(vec) => node {
                type_node: Box::new(node_type::expression(expresions::digits(vec))),
            },
            names::operation(vec) =>{
                let exp = self.clone().parse_expression();
                dbg!(&exp);
                self = exp.1;
                match String::from_utf8_lossy(&vec).to_string().as_str() {
                    "-" => {
                        
                return (node { type_node: Box::new(node_type::expression(expresions::unop(unops::negative, exp.0)))},self);
                    }
                    "not" => return (node{ type_node: Box::new(node_type::expression(expresions::unop(unops::not_node, exp.0)))},self),
                    a => panic!("Expected Unops, got {}",a)
                }
            }

            names::EOF => panic!("EOF"),
            a => panic!(
                "Wrong expression {}",
                String::from_utf8_lossy(&a.value()).to_string()
            ),
        };

        let next_token = match self.tokens.last() {
            Some(a) => a,
            None => return (current_node, self),
        };
        match next_token {
            names::operation(a) => {
                let bin = self.clone().parse_operation(current_node);
                self = bin.1;
                return (bin.0, self);
            }
            names::variable(a) =>{
                if next_token.clone().value() == binops::and.value() || next_token.clone().value() == binops::in_node.value() || next_token.clone().value() == binops::or.value() {
                let bin = self.clone().parse_operation(current_node);
                self = bin.1;
                return (bin.0, self);
                }
            }
            _ => (),
        }
        return (current_node, self);
    }

    fn parse_operation(mut self, current: node) -> (node, Self) {
        let token = self.tokens.pop().expect("EOF").value();
        println!("{}", String::from_utf8_lossy(&token).to_string());
        match String::from_utf8_lossy(&token).to_string().as_str() {
            "+" => {
                println!("plus");
                let bin = self.clone().parse_expression();
                self = bin.1;
                return (
                    node {
                        type_node: Box::new(node_type::expression(expresions::binop(
                            current,
                            binops::plus,
                            bin.0,
                        ))),
                    },
                    self,
                );
            }
            "-" => {
                println!("minus");
                let bin = self.clone().parse_expression();
                self = bin.1;
                return (
                    node {
                        type_node: Box::new(node_type::expression(expresions::binop(
                            current,
                            binops::minus,
                            bin.0,
                        ))),
                    },
                    self,
                );
            }
            "*" => {
                println!("mult");
                let bin = self.clone().parse_expression();
                self = bin.1;
                return (
                    node {
                        type_node: Box::new(node_type::expression(expresions::binop(
                            current,
                            binops::mult,
                            bin.0,
                        ))),
                    },
                    self,
                );
            }
            "/" => {
                println!("div");
                let bin = self.clone().parse_expression();
                self = bin.1;
                return (
                    node {
                        type_node: Box::new(node_type::expression(expresions::binop(
                            current,
                            binops::div,
                            bin.0,
                        ))),
                    },
                    self,
                );
            }
            "==" => {
                println!("equal");
                let bin = self.clone().parse_expression();
                self = bin.1;
                return (
                    node {
                        type_node: Box::new(node_type::expression(expresions::binop(
                            current,
                            binops::equal,
                            bin.0,
                        ))),
                    },
                    self,
                );
            }
            ">=" => {
                println!("mayor_equal");
                let bin = self.clone().parse_expression();
                self = bin.1;
                return (
                    node {
                        type_node: Box::new(node_type::expression(expresions::binop(
                            current,
                            binops::mayor_equal,
                            bin.0,
                        ))),
                    },
                    self,
                );
            }
            "<=" => {
                println!("minor_equal");
                let bin = self.clone().parse_expression();
                self = bin.1;
                return (
                    node {
                        type_node: Box::new(node_type::expression(expresions::binop(
                            current,
                            binops::minor_equal,
                            bin.0,
                        ))),
                    },
                    self,
                );
            }
            ">" => {
                println!("mayor");
                let bin = self.clone().parse_expression();
                self = bin.1;
                return (
                    node {
                        type_node: Box::new(node_type::expression(expresions::binop(
                            current,
                            binops::mayor,
                            bin.0,
                        ))),
                    },
                    self,
                );
            }
            "<" => {
                println!("minor");
                let bin = self.clone().parse_expression();
                self = bin.1;
                return (
                    node {
                        type_node: Box::new(node_type::expression(expresions::binop(
                            current,
                            binops::minor,
                            bin.0,
                        ))),
                    },
                    self,
                );
            }
            "in" => {
                println!("in");
                let bin = self.clone().parse_expression();
                self = bin.1;
                return (
                    node {
                        type_node: Box::new(node_type::expression(expresions::binop(
                            current,
                            binops::in_node,
                            bin.0,
                        ))),
                    },
                    self,
                );
            }
            "and" => {
                println!("and");
                let bin = self.clone().parse_expression();
                self = bin.1;
                return (
                    node {
                        type_node: Box::new(node_type::expression(expresions::binop(
                            current,
                            binops::and,
                            bin.0,
                        ))),
                    },
                    self,
                );
            }
            "or" => {
                println!("or");
                let bin = self.clone().parse_expression();
                self = bin.1;
                return (
                    node {
                        type_node: Box::new(node_type::expression(expresions::binop(
                            current,
                            binops::or,
                            bin.0,
                        ))),
                    },
                    self,
                );
            }
            _ => panic!("todo"),
        }
    }
}
