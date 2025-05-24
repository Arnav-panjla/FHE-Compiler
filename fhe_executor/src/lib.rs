use fhe_core::*;
use fhe_ir::Op;
use std::collections::HashMap;

pub fn execute(ir: Vec<Op>, inputs: HashMap<String, i64>) {
    let (_pk, sk) = keygen();
    let mut encrypted_vars: HashMap<String, i64> = HashMap::new();

    for op in ir {
        match op {
            Op::Input(name) => {
                let val = inputs[&name];
                encrypted_vars.insert(name, encrypt(val, &sk));
            }
            Op::Const(c) => {
                encrypted_vars.insert("const".into(), encrypt(c, &sk));
            }
            Op::Mul(a, b) => {
                let r = homomorphic_mul(&encrypted_vars[&a], &encrypted_vars[&b]);
                encrypted_vars.insert("mul_result".into(), r);
            }
            Op::Add(a, b) => {
                let r = homomorphic_add(&encrypted_vars[&a], &encrypted_vars[&b]);
                encrypted_vars.insert("z".into(), r);
            }
            Op::Output(var) => {
                let result = decrypt(&encrypted_vars[&var], &sk);
                println!("[OUTPUT] {} = {}", var, result);
            }
        }
    }
}
