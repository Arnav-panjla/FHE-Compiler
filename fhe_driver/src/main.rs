use fhe_compiler::compile_dsl;
use fhe_executor::execute;
use std::fs;

fn main() {
    let source = fs::read_to_string("./samples/samp1.dsl").expect("Could not read DSL file");
    let (ir, inputs) = compile_dsl(&source);
    execute(ir, inputs);
}
