use graphviz_rust::dot_structures::Graph;
use types::node_type;

pub mod types;
pub mod implementations;

#[derive(Debug,Clone)]
pub struct node{
    pub type_node : Box<types::node_type>,
}

impl node {
    #[cfg(test)]
    pub fn generate_model(self) -> String{
        String::new()
    }
}