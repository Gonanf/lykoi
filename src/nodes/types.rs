use super::node;

/*#[derive(Debug, Clone)]
pub enum primitives {
    integer,
    string,
    character,
    none,
    bool,
} */

#[derive(Debug, Clone)]
pub enum node_type {
    variable(Vec<u8>),
    expression(expresions),
    //Exp || Var
    return_node(node),
    statement(statement),
    //{ Lots of expressions | return (optional values) }
    block(Vec<node>),
}

#[derive(Debug, Clone)]
pub enum statement {
    //Expresion | Block | {Elifs} | (Else)
    if_node(node, node, Vec<node>, Option<node>),
    //Expresion | Block
    elif_node(node, node),
    //Block
    else_node(node),
    //Exp | block || From | When | End | block
    for_node(node, Option<node>, Option<node>, node),
    //Exp | block
    while_node(node, node),
    break_node,
    continue_node,
    //Var | "=" | Exp
    assignment(node, node),
}

#[derive(Debug, Clone)]

pub enum binops {
    //binop
    in_node,
    assign,
    plus,
    minus,
    div,
    mult,
    mayor,
    minor,
    mayor_equal,
    minor_equal,
    equal,
    or,
    and,
}

impl binops{
    pub fn get_priority(self) -> u8{
        return self as u8;
    }
}

#[derive(Debug, Clone)]

pub enum unops {
    negative,
    not_node,
}

#[derive(Debug, Clone)]

pub enum expresions {
    //Exp | Binop | Exp
    binop(node, binops, node),
    //Unop | Exp
    unop(unops, node),
    true_exp,
    false_exp,
    none_exp,
    digits(Vec<u8>),
    literal(Vec<u8>),
    //Variable | Expression
    node(node),
}
