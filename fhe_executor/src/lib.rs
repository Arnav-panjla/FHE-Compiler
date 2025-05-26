use fhe_core::*;
use fhe_ir::Op;
use std::collections::HashMap;

pub fn execute(ir: Vec<Op>, inputs: HashMap<String, i64>) -> Option<i64>{
    let (_pk, sk) = keygen();
    let mut encrypted_vars: HashMap<String, i64> = HashMap::new();
    let mut const_count = 0;
    let mut output_val = None;

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
                output_val = Some(val);
            }
        }
    }
    output_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_output() {
        let mut inputs = HashMap::new();
        inputs.insert("x".to_string(), 42);
        
        let ops = vec![
            Op::Input("x".to_string()),
            Op::Output("x".to_string()),
        ];
        
        let result = execute(ops, inputs);
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_addition() {
        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), 5);
        inputs.insert("b".to_string(), 3);
        
        let ops = vec![
            Op::Input("a".to_string()),
            Op::Input("b".to_string()),
            Op::Add("a".to_string(), "b".to_string()),
            Op::Output("add_result".to_string()),
        ];

        let result = execute(ops, inputs);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_multiplication() {
        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), 4);
        inputs.insert("b".to_string(), 6);
        
        let ops = vec![
            Op::Input("a".to_string()),
            Op::Input("b".to_string()),
            Op::Mul("a".to_string(), "b".to_string()),
            Op::Output("mul_result".to_string()),
        ];

        let result = execute(ops, inputs);
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_constants() {
        let inputs = HashMap::new();
        
        let ops = vec![
            Op::Const(10),
            Op::Output("const_0".to_string()),
        ];

        let result = execute(ops, inputs);
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_complex_expression() {
        let mut inputs = HashMap::new();
        inputs.insert("x".to_string(), 5);
        
        let ops = vec![
            Op::Input("x".to_string()),
            Op::Const(3),
            Op::Mul("x".to_string(), "const_0".to_string()),
            Op::Const(2),
            Op::Add("mul_result".to_string(), "const_1".to_string()),
            Op::Output("add_result".to_string()),
        ];

        let result = execute(ops, inputs);
        assert_eq!(result, Some(17));
    }
}
