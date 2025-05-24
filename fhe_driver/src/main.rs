use fhe_compiler::compile_dsl;
use fhe_executor::execute;
use std::collections::HashMap;

fn main() {
    let ir = compile_dsl();

    let mut inputs = HashMap::new();
    inputs.insert("x".into(), 2);
    inputs.insert("y".into(), 3);

    execute(ir, inputs);
}
