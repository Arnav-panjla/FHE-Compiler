use fhe_ir::Op;

#[derive(Debug)]
pub enum Expr {
    Var(String),
    Const(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub struct Statement {
    pub var: String,
    pub expr: Expr,
}


pub fn parse_dsl(input: &str) -> Statement {
    // Only supports:
    // let z = x * y + 3;

    let tokens: Vec<String> = input
        .replace(";", "")
        .replace("=", " = ")
        .replace("+", " + ")
        .replace("*", " * ")
        .split_whitespace()
        .map(String::from)
        .collect();

    assert_eq!(tokens[0], "let");
    let var = tokens[1].clone();
    assert_eq!(tokens[2], "=");

    // Very naive: x * y + 3
    let left = Expr::Mul(
        Box::new(Expr::Var(tokens[3].clone())),
        Box::new(Expr::Var(tokens[5].clone())),
    );
    let expr = Expr::Add(Box::new(left), Box::new(Expr::Const(tokens[7].parse().unwrap())));

    Statement { var, expr }
}
