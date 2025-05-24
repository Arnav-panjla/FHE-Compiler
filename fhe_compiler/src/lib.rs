use fhe_ir::Op;

pub fn compile_dsl() -> Vec<Op> {
    vec![
        Op::Input("x".into()),
        Op::Input("y".into()),
        Op::Const(3),
        Op::Mul("x".into(), "y".into()),               
        Op::Add("mul_result".into(), "const_0".into()),   
        Op::Output("add_result".into()),
    ]
}
