pub mod tokenizer {
    use std::borrow::BorrowMut;

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum token {
        digits(u8),
        literal_char(u8),
        literal_dec,
        EOF,
        space,
        newline,
        equal,
        minor,
        mayor,
        minus,
        plus,
        mult,
        div,
        left_bracket,
        right_bracket,
    }
    impl token {
        pub fn search_token(token_var: u8) -> Option<token> {
            match (token_var) {
                b'\"' => Some(token::literal_dec),
                b'\0' => Some(token::EOF),
                b' ' => Some(token::space),
                b'\n' => Some(token::newline),
                b'=' => Some(token::equal),
                b'<' => Some(token::minor),
                b'>' => Some(token::mayor),
                b'-' => Some(token::minus),
                b'+' => Some(token::plus),
                b'*' => Some(token::mult),
                b'/' => Some(token::div),
                b'{' => Some(token::left_bracket),
                b'}' => Some(token::right_bracket),
                b'0'..=b'9' => Some(token::digits(token_var)),
                _ => Some(token::literal_char(token_var)),
            }
        }

        pub fn value(self) -> u8 {
            match self {
                token::digits(a) => a,
                token::literal_char(a) => a,
                token::literal_dec => b'\"',
                token::EOF => b'\0',
                token::space => b' ',
                token::newline => b'\n',
                token::equal => b'=',
                token::minor => b'<',
                token::mayor => b'>',
                token::minus => b'-',
                token::plus => b'+',
                token::mult => b'*',
                token::div => b'/',
                token::left_bracket => b'{',
                token::right_bracket => b'}',
            }
        }
    }

    /*if_token,
    else_token,
    elif_token,
    while_token,
    for_token,
    return_token,
    break_token,
    continue_token,
    none_token,
    true_token,
    false_token,
     variable(Vec<u8>),*/
    #[derive(Debug, Clone, PartialEq)]
    pub enum names {
        variable(Vec<u8>),
        literal(Vec<u8>),
        digits(Vec<u8>),
        EOF,
        operation(Vec<u8>),
        left_bracket,
        right_bracket,
    }

    impl names {
        pub fn agroup_tokens(tokens: Vec<token>) -> Vec<names> {
            let mut group_tokens: Vec<names> = Vec::new();
            let mut buffered_token: token = token::EOF;
            for i in tokens {
                if group_tokens.len() > 0 {
                    let mut last = group_tokens.pop().unwrap();
                    match last {
                        names::literal(ref mut a) => {
                            if (a.len() == 1) || a.last().unwrap() != &token::literal_dec.value() {
                                a.push(i.value());
                                group_tokens.push(last);
                                continue;
                            }
                        }

                        names::digits(ref mut a) => {
                            if (buffered_token != token::space && buffered_token != token::newline)
                            {
                                match i {
                                    token::digits(b) => {
                                        a.push(b);
                                        group_tokens.push(last);
                                        continue;
                                    }
                                    _ => (),
                                }
                            }
                        }

                        names::variable(ref mut a) => {
                            if (buffered_token != token::space && buffered_token != token::newline)
                            {
                                match i {
                                    token::literal_char(b) => {
                                        a.push(b);
                                        group_tokens.push(last);
                                        continue;
                                    }
                                    _ => (),
                                }
                            }
                        }

                        names::operation(ref mut a) => match i {
                            token::equal
                            | token::minor
                            | token::mayor
                            | token::minus
                            | token::plus
                            | token::mult
                            | token::div => {
                                a.push(i.value());
                                group_tokens.push(last);
                                continue;
                            }
                            _ => (),
                        },

                        _ => (),
                    }
                    group_tokens.push(last);
                }
                buffered_token = i;

                match i {
                    token::literal_dec => group_tokens.push(names::literal(vec![i.value()])),
                    token::digits(a) => group_tokens.push(names::digits(vec![a])),
                    token::literal_char(a) => group_tokens.push(names::variable(vec![a])),
                    token::EOF => return group_tokens,
                    token::equal
                    | token::minor
                    | token::mayor
                    | token::minus
                    | token::plus
                    | token::mult
                    | token::div => group_tokens.push(names::operation(vec![i.value()])),
                    token::left_bracket => group_tokens.push(names::left_bracket),
                    token::right_bracket => group_tokens.push(names::right_bracket),
                    _ => (),
                }
            }
            return group_tokens;
        }

        pub fn value(self) -> Vec<u8>{
            match self {
                names::variable(vec) => vec,
                names::literal(vec) => vec,
                names::digits(vec) => vec,
                names::EOF => b"\0".to_vec(),
                names::operation(vec) => vec,
                names::left_bracket => b"{".to_vec(),
                names::right_bracket => b"}".to_vec(),
            }
        }
    }

}

pub mod parser{
    use core::str;
    use std::os::linux::raw::stat;

    use super::tokenizer;

    #[derive(Debug)]
    
    enum node_type{
        //{ Lots of expressions | return (optional values) }
        block(Vec<node>),
        statement(statement),
        //Exp || Var
        return_node(node),
        expression(expresions),
        digits(Vec<u8>),
        literal(Vec<u8>),
        variable(Vec<u8>),
    }


    impl node_type {
        pub fn value(self) -> Vec<u8>{
            match self {
                node_type::block(vec) => b"{}".to_vec(),
                node_type::statement(statement) => statement.value(),
                node_type::return_node(node) => b"return".to_vec(),
                node_type::expression(node) => node.value(),
                node_type::digits(vec) => vec,
                node_type::literal(vec) => vec,
                node_type::variable(vec) => vec,
            }
        }

        pub fn describe(self) -> Vec<u8>{
            match self {
                node_type::block(vec) => b"block".to_vec(),
                node_type::statement(statement) => b"statement".to_vec(),
                node_type::expression(node) => b"expression".to_vec(),
                node_type::digits(vec) => b"digits".to_vec(),
                node_type::literal(vec) => b"literal".to_vec(),
                node_type::variable(vec) => b"variable".to_vec(),

                a => a.value(), 
            }
        }

        pub fn is_reserved(value: &Vec<u8>) -> Option<String>{
            match String::from_utf8_lossy(&value).to_string().as_str() {
                "return" => Some(String::from("return")),
                _ => None
            }
        }
    }


    #[derive(Debug)]
    enum statement{
                //Expresion | Block
                if_node(node,node),
                //Expresion | Block
                elif_node(node,node),
                //Block
                else_node(node),
                //"in" | Var | Exp
                for_node(node,node,node),
                //Exp | block
                while_node(node,node),
                break_node,
                continue_node,
                //Var | "=" | Exp
                assignment(node,node)
    }

    impl statement {
        pub fn value(self) -> Vec<u8>{
            match self {
                statement::if_node(node, node1) => b"if".to_vec(),
                statement::elif_node(node, node1) => b"elif".to_vec(),
                statement::else_node(node) => b"else".to_vec(),
                statement::for_node(node, node1, node2) => b"for".to_vec(),
                statement::while_node(node, _) => b"while".to_vec(),
                statement::break_node => b"break".to_vec(),
                statement::continue_node => b"continue".to_vec(),
                statement::assignment(node, node1) => b"=".to_vec(),
            }
        }

        pub fn describe(self) -> Vec<u8>{
            match self {
                statement::assignment(node, node1) => b"assignment".to_vec(),
                a => a.value()
            }
        }

        pub fn is_reserved(value: &Vec<u8>) -> Option<String>{
            match String::from_utf8_lossy(&value).to_string().as_str() {
                "if" => return Some(String::from("if")),
                "elif" => return Some(String::from("elif")),
                "else"  => return Some(String::from("else")),
                "for" => return Some(String::from("for")),
                "while" => return Some(String::from("while")),
                "break" => return Some(String::from("break")),
                "continue" => return Some(String::from("continue")),
                _ => return None
            }
        }
    }

    #[derive(Debug)]

    enum binops{
        //binop
        mayor,
        minor,
        mayor_equal,
        minor_equal,
        equal,
        in_node,
        assign,
        plus,
        minus,
        div,
        mult,
        and,
        or,
    }

    #[derive(Debug)]

    enum unops{
        negative,
        not_node
    }

    impl unops {
        pub fn value(self) -> Vec<u8>{
            match self {
                unops::negative => b"-".to_vec(),
            unops::not_node => b"not".to_vec(),
            }
        }

        pub fn describe(self) -> Vec<u8>{
            match self {
                unops::negative => b"negative".to_vec(),
            unops::not_node => b"not".to_vec(),
            }
        }

        pub fn is_reserved(value: &Vec<u8>) -> Option<unops>{
            match String::from_utf8_lossy(&value).to_string().as_str() {
                "-" => Some(unops::negative),
                "not" => Some(unops::not_node),
                _ => None
            }
        }
    }

    impl binops {
        pub fn value(self) -> Vec<u8>{
            match self {
                binops::mayor => b">".to_vec(),
                binops::minor => b"<".to_vec(),
                binops::mayor_equal => b">=".to_vec(),
                binops::minor_equal => b"<=".to_vec(),
                binops::equal => b"==".to_vec(),
                binops::in_node => b"in".to_vec(),
                binops::assign => b"=".to_vec(),
                binops::plus => b"+".to_vec(),
                binops::minus => b"-".to_vec(),
                binops::mult => b"*".to_vec(),
                binops::div => b"/".to_vec(),
                binops::and => b"and".to_vec(),
                binops::or => b"or".to_vec(),
            }
        }

        pub fn is_reserved(value: &Vec<u8>) -> Option<binops>{
            match String::from_utf8_lossy(&value).to_string().as_str() {
                ">" => Some(binops::mayor),
                "<" => Some(binops::minor),
                ">=" => Some(binops::mayor_equal),
                "<=" => Some(binops::minor_equal),
                "==" => Some(binops::equal),
                "in" => Some(binops::in_node),
                "=" => Some(binops::assign),
                "+" => Some(binops::plus),
                "*" => Some(binops::mult),
                "/" => Some(binops::div),
                "and" => Some(binops::and),
                "or" => Some(binops::or),
                "-" => Some(binops::minus),
                _ => None
            }
        }

        pub fn describe(self) -> Vec<u8>{
            match self {
                binops::mayor => b"mayor".to_vec(),
                binops::minor => b"minor".to_vec(),
                binops::mayor_equal => b"mayor equal".to_vec(),
                binops::minor_equal => b"minor equal".to_vec(),
                binops::equal => b"equal".to_vec(),
                binops::in_node => b"in".to_vec(),
                binops::assign => b"assign".to_vec(),
                binops::plus => b"plus".to_vec(),
                binops::mult => b"multiplication".to_vec(),
                binops::div => b"divicion".to_vec(),
                binops::and => b"and".to_vec(),
                binops::or => b"or".to_vec(),
                binops::minus => b"minus".to_vec(),
            }
        }
    }

    #[derive(Debug)]

    enum expresions{
//Exp | Binop | Exp
binop(node,binops,node),
//Unop | Exp
unop(unops,node),
    true_exp,
    false_exp,
    none_exp,
    //Variable | Expression | Digits | Literals
    node(node),
    }

    impl expresions {
        pub fn value(self) -> Vec<u8>{
            match self {
                expresions::binop(node, binops, node1) => binops.value(),
                expresions::unop(unops, node) => unops.value(),
                expresions::true_exp => b"true".to_vec(),
                expresions::false_exp => b"false".to_vec(),
                expresions::none_exp => b"none".to_vec(),
                expresions::node(node) => node.type_node.value(),
            }
        }
        
        pub fn describe(self) -> Vec<u8>{
            match self {
                expresions::binop(node, binops, node1) => b"Binary operation".to_vec(),
                expresions::unop(unops, node) => b"Unary operation".to_vec(),
                a => a.value()
            }
        }

        pub fn is_reserved(value: &Vec<u8>) -> Option<expresions>{
            match String::from_utf8_lossy(&value).to_string().as_str() {
                "true" => Some(expresions::true_exp),
                "false" => Some(expresions::false_exp),
                "none" => Some(expresions::none_exp),
                _ => None
            }
        }
    }

    pub struct AST_parser{
        tokens: Vec<tokenizer::names>,
        AST : node,
    }

    #[derive(Debug)]

    pub struct node{
        pub type_node : Box<node_type>,
    }

    pub enum AST_Errors {
        sintaxis(String,String)
    }

    impl AST_parser {
        pub fn new_from(tokens_origin: Vec<tokenizer::names>) -> Self{
            let mut tokens = tokens_origin;
            tokens.reverse();
            return Self{ tokens, AST: node { type_node: Box::new(node_type::block(Vec::new())) } };
        }

        pub fn parse_tokens(mut self) -> Result<node,AST_Errors>{
            while let Some(value) = self.tokens.last() {
                match value {
                    tokenizer::names::variable(vec) => {Self::search_for_reserved(value.clone()); self.tokens.pop()},
                    tokenizer::names::literal(vec) => return Err(AST_Errors::sintaxis("200".to_owned(),"Good".to_owned())),
                    tokenizer::names::digits(vec) => return Err(AST_Errors::sintaxis("200".to_owned(),"Good".to_owned())),
                    tokenizer::names::EOF => return Ok(self.AST),
                    tokenizer::names::operation(vec) => return Err(AST_Errors::sintaxis("200".to_owned(),"Good".to_owned())),
                    tokenizer::names::left_bracket => return Err(AST_Errors::sintaxis("200".to_owned(),"Good".to_owned())),
                    tokenizer::names::right_bracket => return Err(AST_Errors::sintaxis("200".to_owned(),"Good".to_owned())),
                };
            }
            return Err(AST_Errors::sintaxis("200".to_owned(),"Good".to_owned()));
        }

        fn peek(self) -> Option<tokenizer::names>{
            return self.tokens.last().cloned();
        }

        fn search_for_reserved(token : tokenizer::names){
            dbg!(statement::is_reserved(&token.clone().value()));
            dbg!(binops::is_reserved(&token.clone().value())); 
            dbg!(unops::is_reserved(&token.clone().value())); 
            dbg!(node_type::is_reserved(&token.clone().value())); 
            dbg!(expresions::is_reserved(&token.value())); 
        }

    }

}
