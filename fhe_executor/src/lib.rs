use fhe_core::*;
use fhe_ir::Op;
use std::collections::HashMap;

pub fn execute(ir: Vec<Op>, inputs: HashMap<String, i64>) {
    let (_pk, sk) = keygen();
    let mut encrypted_vars: HashMap<String, i64> = HashMap::new();
    let mut const_count = 0;

    for op in ir {
        match op {
            Op::Input(name) => {
                let val = inputs[&name];
                let enc = encrypt(val, &sk);
                println!("Encrypted input {} = {:?}", name, enc);
                encrypted_vars.insert(name, enc);
            }
            Op::Const(val) => {
                let name = format!("const_{}", const_count);
                const_count += 1;
                let enc = encrypt(val, &sk);
                println!("Encrypted const {} = {:?}", name, enc);
                encrypted_vars.insert(name, enc);
            }
            Op::Mul(a, b) => {
                let r = homomorphic_mul(&encrypted_vars[&a], &encrypted_vars[&b]);
                println!("Encrypted({}) * Encrypted({}) -> {:?}", a, b, r);
                encrypted_vars.insert("mul_result".into(), r);
            }
            Op::Add(a, b) => {
                let r = homomorphic_add(&encrypted_vars[&a], &encrypted_vars[&b]);
                println!("Encrypted({}) + Encrypted({}) -> {:?}", a, b, r);
                encrypted_vars.insert("add_result".into(), r);
            }
            Op::Output(name) => {
                let val = decrypt(encrypted_vars.get(&name).expect("Missing output var"), &sk);
                println!("\n[OUTPUT] {} = {}", name, val);
            }
        }
    }
}
