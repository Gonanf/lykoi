mod compiler;
mod structure;
fn main() {
    compiler::first_parse(compiler::agroup(compiler::get_types()));
    
}
