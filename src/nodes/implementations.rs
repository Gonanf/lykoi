use std::collections::HashMap;

use super::types::{self, node_type,statement,expresions,binops,unops};



impl types::node_type {

    pub fn value(self) -> Vec<u8>{
        match self {
            node_type::block(vec) => b"{}".to_vec(),
            node_type::statement(statement) => statement.value(),
            node_type::return_node(node) => b"return".to_vec(),
            node_type::expression(node) => node.value(),
            node_type::variable(vec) => vec,
        }
    }

    pub fn describe(self) -> Vec<u8>{
        match self {
            node_type::block(vec) => b"block".to_vec(),
            node_type::statement(statement) => b"statement".to_vec(),
            node_type::expression(node) => b"expression".to_vec(),
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



impl expresions {
    pub fn value(self) -> Vec<u8>{
        match self {
            expresions::binop(node, binops, node1) => binops.value(),
            expresions::unop(unops, node) => unops.value(),
            expresions::true_exp => b"true".to_vec(),
            expresions::false_exp => b"false".to_vec(),
            expresions::none_exp => b"none".to_vec(),
            expresions::node(node) => node.type_node.value(),
            expresions::digits(vec) => vec,
            expresions::literal(vec) => vec,
        }
    }
    
    pub fn describe(self) -> Vec<u8>{
        match self {
            expresions::binop(node, binops, node1) => b"Binary operation".to_vec(),
            expresions::unop(unops, node) => b"Unary operation".to_vec(),
            expresions::digits(vec) => b"digits".to_vec(),
            expresions::literal(a) => b"literal".to_vec(),
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