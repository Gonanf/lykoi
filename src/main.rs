mod compiler;
mod nodes;
fn main() {
    compiler::first_parse(compiler::agroup(compiler::get_types()));
}
