use core::panic;

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
}

//Mejorar e implementar en todos los parsers
#[derive(Debug)]
pub enum AST_Errors {
    sintaxis(String, String),
}

impl AST_parser {
    pub fn new_from(tokens_origin: Vec<tokenizer::names>) -> Self {
        let mut tokens = tokens_origin;
        tokens.reverse();
        return Self { tokens };
    }

    fn peek(self) -> Option<tokenizer::names> {
        return self.tokens.last().cloned();
    }

    //To optimize
    fn parse_variable(mut self, token: &Vec<u8>, line: u32, col: u32) -> (node, Self) {
        match String::from_utf8_lossy(&token).to_string().as_str() {
            "return" => {
                let exp = self.tokens.pop().expect("EOF");
                let bin = self.clone().parse_expression();
                self = bin.1;
                return (
                    node {
                        type_node: Box::new(node_type::return_node(bin.0)),
                        line,
                        col,
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
                            let tok = self.tokens.pop().expect("EOF");
                            println!("find elif");
                            let val = self.clone().parse_variable(
                                &b"elif".to_vec(),
                                tok.clone().get_pos().0,
                                tok.get_pos().1,
                            );
                            self = val.1;
                            elif_vec.push(val.0);
                        }
                        _ => break,
                    }
                }

                let mut else_node = None;
                self.tokens.pop();
                if let Some(value) = self.tokens.last() {
                    dbg!(&value);
                    if value.clone().value()
                        == statement::else_node(node {
                            type_node: Box::new(node_type::expression(expresions::none_exp)),
                            line,
                            col,
                        })
                        .value()
                    {
                        let tok = self.tokens.pop().expect("EOF");
                        let temp_else = self.clone().parse_variable(
                            &statement::else_node(node {
                                type_node: Box::new(node_type::expression(expresions::none_exp)),
                                line,
                                col,
                            })
                            .value(),
                            tok.clone().get_pos().0,
                            tok.get_pos().1,
                        );
                        self = temp_else.1;
                        else_node = Some(temp_else.0);
                    }
                }
                return (
                    node {
                        type_node: Box::new(node_type::statement(statement::if_node(
                            exp.0, block.0, elif_vec, else_node,
                        ))),
                        line,
                        col,
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
                        line,
                        col,
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
                        line,
                        col,
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
                            exp.0, None, None, block.0,
                        ))),
                        line,
                        col,
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
                            exp.0, block.0,
                        ))),
                        line,
                        col,
                    },
                    self,
                );
            }

            a => {
                return (
                    node {
                        type_node: Box::new(node_type::variable(token.to_vec())),
                        line,
                        col,
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
                tokenizer::names::variable(vec, line, col) => {
                    let m = self.clone().parse_variable(&vec, line, col);
                    self = m.1;
                    block.push(m.0);
                    self.tokens.pop();
                }
                tokenizer::names::literal(..) | tokenizer::names::digits(..) => {
                    let m = self.clone().parse_expression();
                    self = m.1;
                    block.push(m.0);
                    self.tokens.pop();
                }

                tokenizer::names::EOF(..) => (),
                tokenizer::names::operation(..) => {
                    let m = self.clone().parse_expression();
                    self = m.1;
                    block.push(m.0);
                    self.tokens.pop();
                }

                tokenizer::names::left_bracket(..) => {
                    self.tokens.pop().expect("EOF");

                    if block.len() != 0 {
                        let bin = self.clone().parse_block();
                        self = bin.1;
                        block.push(bin.0);
                    }
                }

                tokenizer::names::right_bracket(line, col) => {
                    return (
                        node {
                            type_node: Box::new(node_type::block(block)),
                            line,
                            col,
                        },
                        self,
                    );
                }

                _ => todo!(),
            };
        }
        return (
            node {
                type_node: Box::new(node_type::block(block)),
                line: 1,
                col: 1,
            },
            self,
        );
    }

    fn parse_expression(mut self) -> (node, Self) {
        let token = self.tokens.pop().expect("EOF");
        let current_node: node = match token {
            names::variable(vec, line, col) => node {
                type_node: Box::new(node_type::variable(vec)),
                line,
                col,
            },
            names::literal(vec, line, col) => node {
                type_node: Box::new(node_type::expression(expresions::literal(vec))),
                line,
                col,
            },
            names::digits(vec, line, col) => node {
                type_node: Box::new(node_type::expression(expresions::digits(vec))),
                line,
                col,
            },
            names::operation(vec, line, col) => {
                let exp = self.clone().parse_expression();
                dbg!(&exp);
                self = exp.1;
                if let Some(val) = unops::is_reserved(&vec) {
                    return (
                        node {
                            type_node: Box::new(node_type::expression(expresions::unop(
                                val, exp.0,
                            ))),
                            line,
                            col,
                        },
                        self,
                    );
                } else {
                    panic!("Expected Unops, got {}", String::from_utf8_lossy(&vec));
                }
            }

            names::EOF(..) => panic!("EOF"),
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
            names::operation(..) => {
                let bin = self.clone().parse_operation(current_node);
                self = bin.1;
                return (bin.0, self);
            }
            names::variable(..) => {
                if let Some(val) = binops::is_reserved(&next_token.clone().value()) {
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
        if let Some(val) = binops::is_reserved(&token) {
            println!(
                "{} ({})",
                String::from_utf8_lossy(&val.clone().value()),
                String::from_utf8_lossy(&val.clone().describe())
            );
            let bin = self.clone().parse_expression();
            self = bin.clone().1;
            return (
                node {
                    type_node: Box::new(node_type::expression(expresions::binop(
                        current,
                        val,
                        bin.clone().0,
                    ))),
                    line: bin.clone().0.line,
                    col: bin.0.col,
                },
                self,
            );
        } else {
            panic!("Expected expression");
        }
    }
}
