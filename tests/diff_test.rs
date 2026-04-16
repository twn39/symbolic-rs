#[cfg(test)]
mod test_diff {
    use symbolic_rs::expr::MathExpr;
    use symbolic_rs::rules::{differentiate, simplify};
    use egg::RecExpr;

    fn diff_str(s: &str, var: &str) -> String {
        let expr: RecExpr<MathExpr> = s.parse().unwrap();
        differentiate(&expr, var).to_string()
    }

    #[test]
    fn run() {
        println!("diff(-x, x) = {}", diff_str("(neg x)", "x"));
        println!("diff(y, x) = {}", diff_str("y", "x"));
        println!("diff(sqrt(x), x) = {}", diff_str("(sqrt x)", "x"));
    }
}
