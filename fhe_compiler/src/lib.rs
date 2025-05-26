pub mod parser;
pub mod translator;

use parser::parse_dsl;
use translator::compile_ast;
use fhe_ir::Op;

pub fn compile_dsl(input: &str) -> Vec<Op> {
    let ast = parse_dsl(input).unwrap();
    compile_ast(ast)
}
