use types::node_type;

pub mod codegen;
pub mod data;
pub mod implementations;
pub mod interpreter;
pub mod types;

#[derive(Debug, Clone)]
pub struct node {
    pub type_node: Box<types::node_type>,
    pub line: u32,
    pub col: u32,
}

impl node {
    #[cfg(test)]
    pub fn generate_model(self) -> String {
        String::new()
    }
}
