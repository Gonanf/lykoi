use types::node_type;

pub mod data;
pub mod implementations;
pub mod types;

#[derive(Debug, Clone)]
pub struct node {
    pub type_node: Box<types::node_type>,
    pub line: u32,
    pub col: u32,
}
