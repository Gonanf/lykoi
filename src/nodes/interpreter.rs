use std::{any::Any, borrow::Borrow, env::var, f32::consts::E, fs::read, iter::repeat};

use super::{node, node_type, types};

struct variables {
    name: Vec<u8>,
    value: values,
}

impl values {
    pub fn iter(self) -> String {
        match self {
            values::string(a) => a,
            values::integer(a) => a.to_string(),
            _ => {
                eprintln!("not a valid ITER");
                return String::new();
            }
        }
    }
}
#[derive(Debug)]
enum values {
    bool(bool),
    string(String),
    integer(i32),
    none,
}

#[derive(Clone)]
pub struct interpreter {
    root: node,
}

impl interpreter {
    pub fn new(nodes: node) -> Self {
        return interpreter { root: nodes };
    }

    pub fn interpretate(self) {
        match self.root.type_node.borrow() {
            node_type::block(..) => Self::interpretate_block(self.clone(), &self.root),
            node_type::statement(..) => Self::interpretate_statement(self.clone(), &self.root),
            node_type::variable(..) => Self::interpretate_variable(self.clone(), &self.root),
            _ => (),
        };
    }

    fn interpretate_block(self, nodes: &node) {
        if let node_type::block(vec) = nodes.type_node.borrow() {
            for (index, value) in vec.iter().enumerate() {
                match value.type_node.borrow() {
                    node_type::statement(..) => Self::interpretate_statement(self.clone(), value),
                    node_type::return_node(..) => return,
                    node_type::variable(..) => Self::interpretate_variable(self.clone(), value),
                    _ => (),
                }
            }
        }
    }

    fn interpretate_expression(self, tok: &node) -> values {
        if let node_type::expression(exp) = tok.type_node.borrow() {
            match exp {
                types::expresions::none_exp => return values::none,
                types::expresions::node(a) => Self::interpretate_expression(self.clone(), a),
                types::expresions::true_exp => return values::bool(true),
                types::expresions::false_exp => return values::bool(false),
                types::expresions::digits(a) => {
                    let mut value: i32 = 0;
                    for (index, val) in a.iter().enumerate() {
                        value = value.pow(index as u32);
                        value += i32::from(val.clone());
                    }
                    return values::integer(value);
                }
                types::expresions::literal(a) => {
                    return values::string(String::from_utf8_lossy(&a).to_string())
                }

                types::expresions::unop(unop, exp) => match unop {
                    types::unops::not_node => {
                        let exp = Self::interpretate_expression(self.clone(), exp);
                        let a = match exp {
                            values::bool(a) => a,
                            _ => return values::none,
                        };
                        return values::bool(!a);
                    }
                    types::unops::negative => {
                        let exp = Self::interpretate_expression(self.clone(), exp);
                        let a = match exp {
                            values::integer(a) => a,
                            _ => return values::none,
                        };
                        return values::integer(-a);
                    }
                },

                types::expresions::binop(expl, binop, expr) => {
                    let expa = Self::interpretate_expression(self.clone(), expl);
                    let expb = Self::interpretate_expression(self.clone(), expr);
                    match binop {
                        types::binops::or => {
                            let a = match expa {
                                values::bool(a) => a,
                                _ => return values::none,
                            };
                            let b = match expb {
                                values::bool(a) => a,
                                _ => return values::none,
                            };
                            return values::bool(a || b);
                        }
                        types::binops::and => {
                            let a = match expa {
                                values::bool(a) => a,
                                _ => return values::none,
                            };
                            let b = match expb {
                                values::bool(a) => a,
                                _ => return values::none,
                            };
                            return values::bool(a && b);
                        }
                        types::binops::div => {
                            let a = match expa {
                                values::integer(a) => a,
                                _ => return values::none,
                            };
                            let b = match expb {
                                values::integer(a) => a,
                                _ => return values::none,
                            };
                            return values::integer(a / b);
                        }
                        types::binops::equal => {
                            if let values::bool(a) = expa {
                                if let values::bool(b) = expb {
                                    return values::bool(a == b);
                                }
                            }

                            if let values::integer(a) = expa {
                                if let values::integer(b) = expb {
                                    return values::bool(a == b);
                                }
                            }

                            return values::none;
                        }
                        types::binops::mayor => {
                            let a = match expa {
                                values::bool(a) => a,
                                _ => return values::none,
                            };
                            let b = match expb {
                                values::bool(a) => a,
                                _ => return values::none,
                            };
                            return values::bool(a > b);
                        }
                        types::binops::minor => {
                            let a = match expa {
                                values::bool(a) => a,
                                _ => return values::none,
                            };
                            let b = match expb {
                                values::bool(a) => a,
                                _ => return values::none,
                            };
                            return values::bool(a < b);
                        }
                        types::binops::mayor_equal => {
                            let a = match expa {
                                values::bool(a) => a,
                                _ => return values::none,
                            };
                            let b = match expb {
                                values::bool(a) => a,
                                _ => return values::none,
                            };
                            return values::bool(a >= b);
                        }
                        types::binops::minor_equal => {
                            let a = match expa {
                                values::bool(a) => a,
                                _ => return values::none,
                            };
                            let b = match expb {
                                values::bool(a) => a,
                                _ => return values::none,
                            };
                            return values::bool(a <= b);
                        }
                        types::binops::plus => {
                            let a = match expa {
                                values::integer(a) => a,
                                _ => return values::none,
                            };
                            let b = match expb {
                                values::integer(a) => a,
                                _ => return values::none,
                            };
                            return values::integer(a + b);
                        }
                        types::binops::minus => {
                            let a = match expa {
                                values::integer(a) => a,
                                _ => return values::none,
                            };
                            let b = match expb {
                                values::integer(a) => a,
                                _ => return values::none,
                            };
                            return values::integer(a - b);
                        }
                        types::binops::mult => {
                            let a = match expa {
                                values::integer(a) => a,
                                _ => return values::none,
                            };
                            let b = match expb {
                                values::integer(a) => a,
                                _ => return values::none,
                            };
                            return values::integer(a * b);
                        }
                        _ => {
                            eprintln!("Bad expression");
                            return values::none;
                        }
                    };
                }
            };
        }
        return values::none;
    }

    fn interpretate_statement(self, tok: &node) {
        if let node_type::statement(stat) = tok.type_node.borrow() {
            match stat {
                types::statement::if_node(exp, block, elifs, elses) => {
                    dbg!(exp);
                    let exp_n = Self::interpretate_expression(self.clone(), exp);
                    dbg!(&exp_n);
                    let condition = match exp_n {
                        values::bool(a) => a,
                        _ => {
                            eprintln!("Condition is not a boolean");
                            return;
                        }
                    };
                    if condition {
                        Self::interpretate_block(self.clone(), block);
                        for i in elifs {
                            Self::interpretate_statement(self.clone(), i);
                        }
                        match elses {
                            Some(a) => Self::interpretate_statement(self.clone(), a),
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }
    }

    fn interpretate_variable(self, tok: &node) {
        todo!()
        /*for i in self.origin {
            match *i.type_node {
                node_type::block(vec) => (),
                node_type::expression(exp) => (),
                node_type::statement(stat) => (),
                node_type::return_node(exp) => (),
                node_type::variable(var) => (),
            }
        } */
    }
}
