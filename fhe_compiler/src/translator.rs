use fhe_ir::Op;
use crate::parser::{Expr, Statement};

pub fn compile_ast(stmt: Statement) -> Vec<Op> {
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

    let result_var = compile_expr(stmt.expr, &mut ops, &mut inputs, &mut const_id, &mut temp_id);
    ops.push(Op::Output(stmt.var));
    ops
}
