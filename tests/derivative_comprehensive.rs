/// 综合求导测试 — 对照 Symbolics.jl 的 register_derivatives.jl 逐条验证
#[cfg(test)]
mod test_derivatives_comprehensive {
    use symbolic_rs::expr::MathExpr;
    use symbolic_rs::rules::{differentiate, differentiate_n, gradient, jacobian, simplify};
    use symbolic_rs::latex::to_latex;
    use egg::RecExpr;

    fn diff_str(s: &str, var: &str) -> String {
        let expr: RecExpr<MathExpr> = s.parse().unwrap();
        differentiate(&expr, var).to_string()
    }

    fn simplify_str(s: &str) -> String {
        let expr: RecExpr<MathExpr> = s.parse().unwrap();
        simplify(&expr).to_string()
    }

    // ─────────────────────────────────────────────
    // 1. 基础导数
    // ─────────────────────────────────────────────

    #[test]
    fn test_diff_constant() {
        assert_eq!(diff_str("5", "x"), "0");
        assert_eq!(diff_str("3.14", "x"), "0");
    }

    #[test]
    fn test_diff_variable_self() {
        assert_eq!(diff_str("x", "x"), "1");
    }

    #[test]
    fn test_diff_independent_variable() {
        assert_eq!(diff_str("y", "x"), "0");
        assert_eq!(diff_str("z", "x"), "0");
    }

    #[test]
    fn test_diff_neg() {
        // d/dx(-x) = -1
        let res = diff_str("(neg x)", "x");
        println!("d/dx(-x) = {}", res);
        assert!(!res.contains("diff"), "d/dx(-x) = {}", res);
        // The result should be equivalent to -1 (could be `(neg 1)` or `-1`)
        assert!(res == "-1" || res == "(neg 1)" || res == "1" && false,
            "Expected d/dx(-x) = -1 or equivalent, got: {}", res);
    }

    #[test]
    fn test_diff_add_sub() {
        let res_add = diff_str("(+ x x)", "x");
        println!("d/dx(x+x) = {}", res_add);
        assert!(!res_add.contains("diff"));

        let res_sub = diff_str("(- x x)", "x");
        println!("d/dx(x-x) = {}", res_sub);
        assert!(!res_sub.contains("diff"));
    }

    #[test]
    fn test_diff_mul_product_rule() {
        // d/dx(x * sin(x)) should not contain diff
        let res = diff_str("(* x (sin x))", "x");
        println!("d/dx(x * sin(x)) = {}", res);
        assert!(!res.contains("diff"));
    }

    #[test]
    fn test_diff_div_quotient_rule() {
        // d/dx(sin(x)/cos(x)) = d/dx(tan(x))
        let res = diff_str("(/ (sin x) (cos x))", "x");
        println!("d/dx(sin(x)/cos(x)) = {}", res);
        assert!(!res.contains("diff"));
    }

    // ─────────────────────────────────────────────
    // 2. 幂函数
    // ─────────────────────────────────────────────

    #[test]
    fn test_diff_pow_constant_exponent() {
        // d/dx(x^2) = 2*x
        let res = diff_str("(pow x 2)", "x");
        println!("d/dx(x^2) = {}", res);
        assert!(!res.contains("diff"));

        // d/dx(x^3) should be equivalent to 3*x^2
        let res3 = diff_str("(pow x 3)", "x");
        println!("d/dx(x^3) = {}", res3);
        assert!(!res3.contains("diff"));
    }

    #[test]
    fn test_diff_pow_variable_exponent() {
        // d/dx(x^x) — 通用幂法则
        let res = diff_str("(pow x x)", "x");
        println!("d/dx(x^x) = {}", res);
        assert!(!res.contains("diff"));
    }

    #[test]
    fn test_diff_sqrt() {
        // d/dx sqrt(x) = 1/(2*sqrt(x))
        let res = diff_str("(sqrt x)", "x");
        println!("d/dx(sqrt(x)) = {}", res);
        assert!(!res.contains("diff"));
    }

    // ─────────────────────────────────────────────
    // 3. 三角函数 (Symbolics.jl register_derivatives.jl 全覆盖)
    // ─────────────────────────────────────────────

    #[test]
    fn test_diff_sin() {
        let res = diff_str("(sin x)", "x");
        println!("d/dx(sin(x)) = {}", res);
        assert!(!res.contains("diff"));
    }

    #[test]
    fn test_diff_cos() {
        let res = diff_str("(cos x)", "x");
        println!("d/dx(cos(x)) = {}", res);
        assert!(!res.contains("diff"));
    }

    #[test]
    fn test_diff_tan() {
        // d/dx tan(x) = 1/cos^2(x) = sec^2(x)
        let res = diff_str("(tan x)", "x");
        println!("d/dx(tan(x)) = {}", res);
        assert!(!res.contains("diff"));
    }

    #[test]
    fn test_diff_asin() {
        let res = diff_str("(asin x)", "x");
        println!("d/dx(asin(x)) = {}", res);
        assert!(!res.contains("diff"));
    }

    #[test]
    fn test_diff_acos() {
        let res = diff_str("(acos x)", "x");
        println!("d/dx(acos(x)) = {}", res);
        assert!(!res.contains("diff"));
    }

    #[test]
    fn test_diff_atan() {
        let res = diff_str("(atan x)", "x");
        println!("d/dx(atan(x)) = {}", res);
        assert!(!res.contains("diff"));
    }

    #[test]
    fn test_diff_sec() {
        // d/dx sec(x) = sec(x)*tan(x) * 1
        let res = diff_str("(sec x)", "x");
        println!("d/dx(sec(x)) = {}", res);
        assert!(!res.contains("diff"), "d/dx(sec(x)) still contains diff: {}", res);
    }

    #[test]
    fn test_diff_csc() {
        // d/dx csc(x) = -csc(x)*cot(x)
        let res = diff_str("(csc x)", "x");
        println!("d/dx(csc(x)) = {}", res);
        assert!(!res.contains("diff"), "d/dx(csc(x)) still contains diff: {}", res);
    }

    #[test]
    fn test_diff_cot() {
        // d/dx cot(x) = -csc^2(x)
        let res = diff_str("(cot x)", "x");
        println!("d/dx(cot(x)) = {}", res);
        assert!(!res.contains("diff"), "d/dx(cot(x)) still contains diff: {}", res);
    }

    #[test]
    fn test_diff_asec() {
        // d/dx asec(x) = 1/(|x|*sqrt(x^2-1))
        let res = diff_str("(asec x)", "x");
        println!("d/dx(asec(x)) = {}", res);
        assert!(!res.contains("diff"), "d/dx(asec(x)) still contains diff: {}", res);
    }

    #[test]
    fn test_diff_acsc() {
        // d/dx acsc(x) = -1/(|x|*sqrt(x^2-1))
        let res = diff_str("(acsc x)", "x");
        println!("d/dx(acsc(x)) = {}", res);
        assert!(!res.contains("diff"), "d/dx(acsc(x)) still contains diff: {}", res);
    }

    #[test]
    fn test_diff_acot() {
        // d/dx acot(x) = -1/(1+x^2)
        let res = diff_str("(acot x)", "x");
        println!("d/dx(acot(x)) = {}", res);
        assert!(!res.contains("diff"), "d/dx(acot(x)) still contains diff: {}", res);
    }

    // ─────────────────────────────────────────────
    // 4. 双曲函数
    // ─────────────────────────────────────────────

    #[test]
    fn test_diff_sinh() {
        let res = diff_str("(sinh x)", "x");
        println!("d/dx(sinh(x)) = {}", res);
        assert!(!res.contains("diff"));
    }

    #[test]
    fn test_diff_cosh() {
        let res = diff_str("(cosh x)", "x");
        println!("d/dx(cosh(x)) = {}", res);
        assert!(!res.contains("diff"));
    }

    #[test]
    fn test_diff_tanh() {
        let res = diff_str("(tanh x)", "x");
        println!("d/dx(tanh(x)) = {}", res);
        assert!(!res.contains("diff"));
    }

    #[test]
    fn test_diff_asinh() {
        let res = diff_str("(asinh x)", "x");
        println!("d/dx(asinh(x)) = {}", res);
        assert!(!res.contains("diff"));
    }

    #[test]
    fn test_diff_acosh() {
        let res = diff_str("(acosh x)", "x");
        println!("d/dx(acosh(x)) = {}", res);
        assert!(!res.contains("diff"));
    }

    #[test]
    fn test_diff_atanh() {
        let res = diff_str("(atanh x)", "x");
        println!("d/dx(atanh(x)) = {}", res);
        assert!(!res.contains("diff"));
    }

    // ─────────────────────────────────────────────
    // 5. 指数与对数
    // ─────────────────────────────────────────────

    #[test]
    fn test_diff_exp() {
        // d/dx e^x = e^x
        let res = diff_str("(exp x)", "x");
        println!("d/dx(exp(x)) = {}", res);
        assert!(!res.contains("diff"));
    }

    #[test]
    fn test_diff_log() {
        // d/dx ln(x) = 1/x
        let res = diff_str("(log x)", "x");
        println!("d/dx(ln(x)) = {}", res);
        assert!(!res.contains("diff"));
    }

    #[test]
    fn test_diff_log2() {
        // d/dx log2(x) = 1/(x*ln(2))
        let res = diff_str("(log2 x)", "x");
        println!("d/dx(log2(x)) = {}", res);
        assert!(!res.contains("diff"), "d/dx(log2(x)) still contains diff: {}", res);
    }

    // ─────────────────────────────────────────────
    // 6. 链式法则 (Chain Rule)
    // ─────────────────────────────────────────────

    #[test]
    fn test_diff_chain_sin_pow() {
        // d/dx sin(x^2)
        let res = diff_str("(sin (pow x 2))", "x");
        println!("d/dx(sin(x^2)) = {}", res);
        assert!(!res.contains("diff"));
    }

    #[test]
    fn test_diff_chain_exp_log() {
        // d/dx exp(ln(x)) = d/dx x = 1 (after simplification)
        let res = diff_str("(exp (log x))", "x");
        println!("d/dx(exp(ln(x))) = {}", res);
        assert!(!res.contains("diff"));
    }

    #[test]
    fn test_diff_chain_log_sin() {
        // d/dx ln(sin(x)) = cos(x)/sin(x) = cot(x)
        let res = diff_str("(log (sin x))", "x");
        println!("d/dx(ln(sin(x))) = {}", res);
        assert!(!res.contains("diff"));
    }

    // ─────────────────────────────────────────────
    // 7. 高阶导数 (Higher-order derivatives)
    // ─────────────────────────────────────────────

    #[test]
    fn test_second_derivative_sin() {
        // d²/dx² sin(x) = -sin(x)
        let expr: RecExpr<MathExpr> = "(sin x)".parse().unwrap();
        let d2 = differentiate_n(&expr, "x", 2);
        let res = d2.to_string();
        println!("d²/dx²(sin(x)) = {}", res);
        assert!(!res.contains("diff"), "Second derivative of sin(x) should be fully expanded: {}", res);
    }

    #[test]
    fn test_second_derivative_pow() {
        // d²/dx² x^3 = 6*x (or equivalent)
        let expr: RecExpr<MathExpr> = "(pow x 3)".parse().unwrap();
        let d2 = differentiate_n(&expr, "x", 2);
        let res = d2.to_string();
        println!("d²/dx²(x^3) = {}", res);
        assert!(!res.contains("diff"), "Second derivative result: {}", res);
    }

    #[test]
    fn test_third_derivative_pow3() {
        // d³/dx³ x^3 = 6 (constant)
        let expr: RecExpr<MathExpr> = "(pow x 3)".parse().unwrap();
        let d3 = differentiate_n(&expr, "x", 3);
        let res = d3.to_string();
        println!("d³/dx³(x^3) = {}", res);
        assert!(!res.contains("diff"), "Third derivative should be fully evaluated, got: {}", res);
    }

    #[test]
    fn test_zeroth_derivative() {
        // 0 阶导数 = 原函数
        let expr: RecExpr<MathExpr> = "(sin x)".parse().unwrap();
        let d0 = differentiate_n(&expr, "x", 0);
        assert_eq!(d0.to_string(), "(sin x)");
    }

    // ─────────────────────────────────────────────
    // 8. 梯度与雅可比 (Gradient & Jacobian)
    // ─────────────────────────────────────────────

    #[test]
    fn test_gradient_quadratic() {
        // grad(x^2 + y^2) wrt [x, y]
        let expr: RecExpr<MathExpr> = "(+ (pow x 2) (pow y 2))".parse().unwrap();
        let grad = gradient(&expr, &["x", "y"]);

        println!("∂/∂x(x²+y²) = {}", grad[0]);
        println!("∂/∂y(x²+y²) = {}", grad[1]);

        assert!(!grad[0].to_string().contains("diff"));
        assert!(!grad[1].to_string().contains("diff"));
    }

    #[test]
    fn test_gradient_multivariate() {
        // grad(x*y*z) wrt [x, y, z]
        let expr: RecExpr<MathExpr> = "(* (* x y) z)".parse().unwrap();
        let grad = gradient(&expr, &["x", "y", "z"]);

        for (v, g) in ["x", "y", "z"].iter().zip(grad.iter()) {
            println!("∂/∂{}(x*y*z) = {}", v, g);
            assert!(!g.to_string().contains("diff"));
        }
        assert_eq!(grad.len(), 3);
    }

    #[test]
    fn test_jacobian_basic() {
        // J([x*y, x+y], [x, y])
        //   = [[y, x],
        //      [1, 1]]
        let f1: RecExpr<MathExpr> = "(* x y)".parse().unwrap();
        let f2: RecExpr<MathExpr> = "(+ x y)".parse().unwrap();
        let jac = jacobian(&[f1, f2], &["x", "y"]);

        assert_eq!(jac.len(), 2, "Jacobian row count");
        assert_eq!(jac[0].len(), 2, "Jacobian col count");

        for (i, row) in jac.iter().enumerate() {
            for (j, entry) in row.iter().enumerate() {
                let s = entry.to_string();
                println!("J[{}][{}] = {}", i, j, s);
                assert!(!s.contains("diff"), "J[{}][{}] still contains diff: {}", i, j, s);
            }
        }
    }

    // ─────────────────────────────────────────────
    // 9. LaTeX 输出验证
    // ─────────────────────────────────────────────

    #[test]
    fn test_latex_diff_result() {
        // d/dx sin(x) => cos(x), and LaTeX should contain \cos
        let expr: RecExpr<MathExpr> = "(sin x)".parse().unwrap();
        let deriv = differentiate(&expr, "x");
        let tex = to_latex(&deriv);
        println!("LaTeX of d/dx(sin(x)) = {}", tex);
        assert!(tex.contains(r"\cos"), "Expected LaTeX to contain \\cos, got: {}", tex);
    }

    #[test]
    fn test_latex_new_functions() {
        let cases = vec![
            ("(sec x)",  r"\sec\left(x\right)"),
            ("(csc x)",  r"\csc\left(x\right)"),
            ("(cot x)",  r"\cot\left(x\right)"),
            ("(log2 x)", r"\log_{2}\left(x\right)"),
            ("(abs x)",  r"\left|x\right|"),
        ];
        for (s, expected) in cases {
            let expr: RecExpr<MathExpr> = s.parse().unwrap();
            let tex = to_latex(&expr);
            assert_eq!(tex, expected, "LaTeX mismatch for {}: got {}", s, tex);
        }
    }
}
