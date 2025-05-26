use fhe_compiler::compile_dsl;
use fhe_executor::execute;

fn main() {
    let ir = compile_dsl("let a = x * 3 + (y + z) * 3;");
    let mut inputs = std::collections::HashMap::new();
    inputs.insert("x".into(), 2);
    inputs.insert("y".into(), 3);
    inputs.insert("z".into(), 4);

    execute(ir, inputs);
}
