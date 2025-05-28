pub mod parser;
pub mod translator;

use parser::parse_dsl;
use translator::compile_ast;
use fhe_ir::Op;
use std::collections::HashMap;

pub fn compile_dsl(src: &str) -> (Vec<Op>, HashMap<String, i64>) {
    let mut ir = Vec::new();
    let mut inputs = HashMap::new();

    for line in src.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if line.starts_with("input ") {
            if let Some((name, value)) = line["input ".len()..].split_once('=') {
                let var = name.trim().to_string();
                let val = value.trim_end_matches(';').trim().parse::<i64>().unwrap();
                inputs.insert(var, val);
            }
        } else if line.starts_with("let ") {
            let expression = line.trim_end_matches(';');
            let ast = parse_dsl(expression).unwrap();
            let mut ir_ast = compile_ast(ast);
            ir.append(&mut ir_ast);
        } 
        // else if line.starts_with("output ") {
        //     // Parse output a;
        //     let var = line["output ".len()..].trim_end_matches(';').trim().to_string();
        //     ir.push(Op::Output(var));
        // }
    }

    (ir, inputs)
}
