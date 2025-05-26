use fhe_ir::Op;
use crate::parser::{Expr, Statement};
use std::collections::HashSet;

pub fn compile_ast(ast: Statement) -> Vec<Op> {
    let mut ops = Vec::new();
    let mut inputs = std::collections::HashSet::new();
    let mut const_id = 0;
    let mut temp_id = 0;

    fn compile_expr(
        expr: Expr,
        ops: &mut Vec<Op>,
        inputs: &mut std::collections::HashSet<String>,
        const_id: &mut usize,
        temp_id: &mut usize,
    ) -> String {
        match expr {
            Expr::Var(v) => {
                if !inputs.contains(&v) {
                    ops.push(Op::Input(v.clone()));
                    inputs.insert(v.clone());
                }
                v
            }
            Expr::Const(c) => {
                let name = format!("const_{}", *const_id);
                *const_id += 1;
                ops.push(Op::Const(c));
                name
            }
            Expr::Mul(a, b) => {
                let a_name = compile_expr(*a, ops, inputs, const_id, temp_id);
                let b_name = compile_expr(*b, ops, inputs, const_id, temp_id);
                let name = format!("t{}", *temp_id);
                *temp_id += 1;
                ops.push(Op::Mul(a_name.clone(), b_name.clone()));
                name
            }
            Expr::Add(a, b) => {
                let a_name = compile_expr(*a, ops, inputs, const_id, temp_id);
                let b_name = compile_expr(*b, ops, inputs, const_id, temp_id);
                let name = format!("t{}", *temp_id);
                *temp_id += 1;
                ops.push(Op::Add(a_name.clone(), b_name.clone()));
                name
            }
        }
    }

    let result_var = compile_expr(ast.expr, &mut ops, &mut inputs, &mut const_id, &mut temp_id);
    println!("Result variable: {:?}", result_var);
    ops.push(Op::Output(ast.var));
    println!("Ops: {:?}", ops);
    ops
}

#[cfg(test)]
mod tests {
    use super::*;
    use fhe_executor::execute;
    use std::collections::HashMap;

    #[test]
    fn test_simple_variable() {
        let stmt = Statement {
            var: "y".to_string(),
            expr: Expr::Var("x".to_string()),
        };
        let ops = compile_ast(stmt);

        assert_eq!(ops, vec![
            Op::Input("x".to_string()),
            Op::Output("y".to_string()),
        ]);

        // let mut inputs = HashMap::new();
        // inputs.insert("x".to_string(), 42);
        
        // let result = execute(ops, inputs);
        // assert_eq!(result, Some(42));
    }

    #[test]
    fn test_constant() {
        let stmt = Statement {
            var: "y".to_string(),
            expr: Expr::Const(42),
        };
        let ops = compile_ast(stmt);
        assert_eq!(ops, vec![
            Op::Const(42),
            Op::Output("y".to_string()),
        ]);
        
        // let inputs = HashMap::new();
        // let result = execute(ops, inputs);
        // assert_eq!(result, Some(42));
    }

    #[test]
    fn test_multiplication() {
        let stmt = Statement {
            var: "z".to_string(),
            expr: Expr::Mul(
                Box::new(Expr::Var("x".to_string())),
                Box::new(Expr::Var("y".to_string())),
            ),
        };
        let ops = compile_ast(stmt);

        assert_eq!(ops, vec![
            Op::Input("x".to_string()),
            Op::Input("y".to_string()),
            Op::Mul("x".to_string(), "y".to_string()),
            Op::Output("z".to_string()),
        ]);
        
    }

    #[test]
    fn test_addition() {
        let stmt = Statement {
            var: "z".to_string(),
            expr: Expr::Add(
                Box::new(Expr::Var("x".to_string())),
                Box::new(Expr::Var("y".to_string())),
            ),
        };
        let ops = compile_ast(stmt);
        assert_eq!(ops, vec![
            Op::Input("x".to_string()),
            Op::Input("y".to_string()),
            Op::Add("x".to_string(), "y".to_string()),
            Op::Output("z".to_string()),
        ]);
    }

    #[test]
    #[ignore]
    fn test_complex_expression() { // TODO: fix this test
        let stmt = Statement {
            var: "result".to_string(),
            expr: Expr::Add(
                Box::new(Expr::Mul(
                    Box::new(Expr::Var("x".to_string())),
                    Box::new(Expr::Var("y".to_string())),
                )),
                Box::new(Expr::Const(5)),
            ),
        };
        let ops = compile_ast(stmt);
        println!("Ops: {:?}", ops);

        assert_eq!(ops, vec![
            Op::Input("x".to_string()),
            Op::Input("y".to_string()),
            Op::Mul("x".to_string(), "y".to_string()),
            Op::Const(5),
            Op::Add("x".to_string(), "y".to_string()),
            Op::Output("result".to_string()),
        ])
    }
}
