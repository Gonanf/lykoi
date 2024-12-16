mod compiler;
mod nodes;
fn main() {
    compiler::interpretate(compiler::first_parse(compiler::agroup(
        compiler::get_types(),
    )));
}
