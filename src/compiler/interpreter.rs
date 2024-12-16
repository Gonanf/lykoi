use std::{
    any::Any, borrow::Borrow, env::var, f32::consts::E, fmt, fmt::Display, fs::read, iter::repeat,
};

use crate::nodes;

use nodes::{node, types::node_type, types};

#[derive(Debug,Clone)]
struct variables {
    name: String,
    value: values,
}

impl fmt::Display for variables {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            values::none => {
                write!(f, "{} = none", self.name)
            }
            values::bool(a) => {
                write!(f, "{} = {a}", self.name)
            }
            values::string(a) => {
                write!(f, "{} = {a}", self.name)
            }
            values::integer(a) => {
                write!(f, "{} = {a}", self.name)
            }
        }
    }
}

impl values {
    pub fn iterate(self) -> String {
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
#[derive(Debug,Clone)]
enum values {
    bool(bool),
    string(String),
    integer(i32),
    none,
}

pub struct interpreter {
    root: node,
}

impl interpreter {
    pub fn new(nodes: node) -> Self {
        return interpreter { root: nodes };
    }

    pub fn interpretate(self) {
        let root_clone = self.root.clone();
        let mut stack = Vec::new();
        match self.root.type_node.borrow() {
            node_type::block(..) => Self::interpretate_block(&mut stack, &root_clone),
            node_type::statement(..) => Self::interpretate_statement(&mut stack, &root_clone),
            node_type::variable(..) => {
                Self::interpretate_variable(&mut stack, &root_clone);
            }
            _ => (),
        };
    }

    fn interpretate_block(stack: &mut Vec<variables>, nodes: &node) {
        if let node_type::block(vec) = nodes.type_node.borrow() {
            for (index, value) in vec.iter().enumerate() {
                dbg!(&value);
                match value.type_node.borrow() {
                    node_type::statement(..) => Self::interpretate_statement(stack, value),
                    node_type::return_node(..) => return,
                    node_type::variable(..) => {
                        Self::interpretate_variable(stack, value);
                    }
                    _ => (),
                }
            }
        }
    }

    fn interpretate_expression(stack: &mut Vec<variables>, tok: &node) -> values {
        if let node_type::variable(name) = tok.type_node.borrow(){
            for i in stack.clone() {
                if i.name == String::from_utf8_lossy(&name).to_string() {
                    return i.value.clone();
                }
            }
        }
        if let node_type::expression(exp) = tok.type_node.borrow() {
            match exp {
                types::expresions::none_exp => return values::none,
                types::expresions::node(a) => {
                    Self::interpretate_expression(stack, a);
                },
                types::expresions::true_exp => return values::bool(true),
                types::expresions::false_exp => return values::bool(false),
                types::expresions::digits(a) => {
                    let mut value: i32 = 0;
                    for (index, val) in a.iter().enumerate() {
                        value *= 10_i32.pow(index as u32);
                        value += i32::from(val.clone()- b'0') ;
                    }
                    return values::integer(value);
                }
                types::expresions::literal(a) => {
                    return values::string(String::from_utf8_lossy(&a).to_string())
                }

                types::expresions::unop(unop, exp) => match unop {
                    types::unops::not_node => {
                        let exp = Self::interpretate_expression(stack, exp);
                        let a = match exp {
                            values::bool(a) => a,
                            _ => return values::none,
                        };
                        return values::bool(!a);
                    }
                    types::unops::negative => {
                        let exp = Self::interpretate_expression(stack, exp);
                        let a = match exp {
                            values::integer(a) => a,
                            _ => return values::none,
                        };
                        return values::integer(-a);
                    }
                },

                types::expresions::binop(expl, binop, expr) => {
                    let expa = Self::interpretate_expression(stack, expl);
                    let expb = Self::interpretate_expression(stack, expr);
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
                            //dbg!(&expa);
                            //dbg!(&expb);

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
                                values::integer(a) => a,
                                _ => return values::none,
                            };
                            let b = match expb {
                                values::integer(a) => a,
                                _ => return values::none,
                            };
                            return values::bool(a > b);
                        }
                        types::binops::minor => {
                            let a = match expa {
                                values::integer(a) => a,
                                _ => return values::none,
                            };
                            let b = match expb {
                                values::integer(a) => a,
                                _ => return values::none,
                            };
                            return values::bool(a < b);
                        }
                        types::binops::mayor_equal => {
                            let a = match expa {
                                values::integer(a) => a,
                                _ => return values::none,
                            };
                            let b = match expb {
                                values::integer(a) => a,
                                _ => return values::none,
                            };
                            return values::bool(a >= b);
                        }
                        types::binops::minor_equal => {
                            let a = match expa {
                                values::integer(a) => a,
                                _ => return values::none,
                            };
                            let b = match expb {
                                values::integer(a) => a,
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

    fn interpretate_statement(stack: &mut Vec<variables>, tok: &node) {
        if let node_type::statement(stat) = tok.type_node.borrow() {
            match stat {
                types::statement::if_node(exp, block, elifs, elses) => {
                    //dbg!(exp);
                    let exp_n = Self::interpretate_expression(stack, exp);
                    //dbg!(&exp_n);
                    let condition = match exp_n {
                        values::bool(a) => a,
                        _ => {
                            eprintln!("Condition is not a boolean");
                            return;
                        }
                    };
                    if condition {
                        Self::interpretate_block(stack, block);
                        for i in elifs {
                            Self::interpretate_statement(stack, i);
                        }
                        
                    }
                    else{
                        dbg!(elses);
                        match elses {
                            Some(a) => Self::interpretate_statement(stack, a),
                            _ => (),
                        }
                    }
                }
                types::statement::elif_node(exp, block) => {
                    //dbg!(exp);
                    let exp_n = Self::interpretate_expression(stack, exp);
                    //dbg!(&exp_n);
                    let condition = match exp_n {
                        values::bool(a) => a,
                        _ => {
                            eprintln!("Condition is not a boolean");
                            return;
                        }
                    };
                    if condition {
                        Self::interpretate_block(stack, block);
                    }
                }
                types::statement::else_node(block) => {
                    Self::interpretate_block(stack, block);
                }
                types::statement::assignment(a, b) => {
                    let expb = Self::interpretate_expression(stack, b);
                    if let node_type::variable(vala) = a.type_node.borrow() {
                        for i in stack.iter_mut() {
                            if i.name == String::from_utf8_lossy(&vala).to_string() {
                                i.value = expb;
                                return;
                            }
                        }
                        stack.push(variables {
                            name: String::from_utf8_lossy(&vala).to_string(),
                            value: expb,
                        });
                        //dbg!(&stack);
                    }
                }
                _ => (),
            }
        }
    }

    fn interpretate_variable(stack: &mut Vec<variables>, tok: &node) {
        if let node_type::variable(a) = tok.type_node.borrow() {
            for i in stack {
                if i.name == String::from_utf8_lossy(&a).to_string() {
                    println!("{}", i);
                    return;
                }
            }
            eprintln!("Not found");
        }
    }
}
