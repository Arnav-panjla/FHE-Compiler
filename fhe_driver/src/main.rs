use fhe_compiler::compile_dsl;
use fhe_executor::execute;

fn main() {
    let ir = compile_dsl("let z = x * y + 3;");
    let mut inputs = std::collections::HashMap::new();
    inputs.insert("x".into(), 2);
    inputs.insert("y".into(), 3);

    execute(ir, inputs);
}
