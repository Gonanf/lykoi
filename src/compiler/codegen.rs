use std::borrow::Borrow;
use crate::nodes;

use nodes::{node, types::node_type, types};

impl node {
    pub fn codegen(self) {
        match self.type_node.borrow() {
            node_type::block(vec) => Self::codegen_block(self),
            node_type::expression(..) => Self::codegen_expression(self),
            node_type::return_node(..) => Self::codegen_return(self),
            node_type::statement(..) => Self::codegen_statement(self),
            node_type::variable(..) => Self::codegen_variable(self),
        };
    }

    fn codegen_block(block: node) {
        if let node_type::block(vec) = block.type_node.borrow() {
            for i in vec.clone() {
                i.codegen();
            }
        }
    }

    fn codegen_expression(block: node) {
        ()
    }

    fn codegen_return(block: node) {
        ()
    }

    fn codegen_statement(block: node) {
        ()
    }

    fn codegen_variable(block: node) {
        ()
    }
}
