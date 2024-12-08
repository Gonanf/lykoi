pub mod types;
pub mod implementations;

#[derive(Debug,Clone)]
pub struct node{
    pub type_node : Box<types::node_type>,
}