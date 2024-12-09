mod compiler;
mod nodes;
pub mod test;
fn main() {
    compiler::first_parse(compiler::agroup(compiler::get_types()));
}
