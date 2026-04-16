#[cfg(test)]
mod test_diff2 {
    use symbolic_rs::expr::MathExpr;
    use symbolic_rs::rules::{differentiate};
    use egg::RecExpr;

    fn diff_str(s: &str, var: &str) -> String {
        let expr: RecExpr<MathExpr> = s.parse().unwrap();
        differentiate(&expr, var).to_string()
    }

    #[test]
    fn run() {
        println!("diff(pow(x, x), x) = {}", diff_str("(pow x x)", "x"));
        println!("diff(y * x, x) = {}", diff_str("(* y x)", "x"));
        println!("diff(pow(x, 2), x) = {}", diff_str("(pow x 2)", "x"));
        println!("diff(sqrt(x), x) = {}", diff_str("(sqrt x)", "x"));
    }
}
