/// Easy-features 综合测试（对标 Symbolics.jl 功能）
#[cfg(test)]
mod tests_easy_features {
    use symbolic_rs::expr::MathExpr;
    use symbolic_rs::rules::{
        build_function, coeff_of, coeff_of_n, degree, expand, hessian,
        is_affine, is_linear, simplify, substitute, substitute_num, taylor,
        taylor_coeff, Equation,
    };
    use symbolic_rs::latex::to_latex;
    use egg::RecExpr;

    // ─────────────────────────────────────────────
    // 工具函数
    // ─────────────────────────────────────────────

    fn num_in(expr: &RecExpr<MathExpr>) -> Option<f64> {
        match expr.as_ref().last()? {
            MathExpr::Num(n) => Some(n.into_inner()),
            _ => None,
        }
    }

    // ─────────────────────────────────────────────
    // 1. substitute_num — 数值代入
    // ─────────────────────────────────────────────

    #[test]
    fn test_substitute_num_linear() {
        // 2x + 1 at x=3  =>  7
        let expr: RecExpr<MathExpr> = "(+ (* 2 x) 1)".parse().unwrap();
        let result = substitute_num(&expr, "x", 3.0);
        println!("substitute_num((+ (* 2 x) 1), x, 3) = {}", result);
        assert_eq!(num_in(&result), Some(7.0));
    }

    #[test]
    fn test_substitute_num_quadratic() {
        // x^2 + x + 1 at x=2 => 7
        let expr: RecExpr<MathExpr> = "(+ (+ (pow x 2) x) 1)".parse().unwrap();
        let result = substitute_num(&expr, "x", 2.0);
        println!("P(2) = {}", result);
        assert_eq!(num_in(&result), Some(7.0));
    }

    #[test]
    fn test_substitute_num_trig() {
        // sin(x) at x=0 => 0
        let expr: RecExpr<MathExpr> = "(sin x)".parse().unwrap();
        let result = substitute_num(&expr, "x", 0.0);
        println!("sin(0) = {}", result);
        assert_eq!(num_in(&result), Some(0.0));

        // cos(x) at x=0 => 1
        let expr2: RecExpr<MathExpr> = "(cos x)".parse().unwrap();
        let result2 = substitute_num(&expr2, "x", 0.0);
        assert_eq!(num_in(&result2), Some(1.0));
    }

    #[test]
    fn test_substitute_num_independent_var() {
        // sin(y) at x=1 => sin(y)  (未触碰)
        let expr: RecExpr<MathExpr> = "(sin y)".parse().unwrap();
        let result = substitute_num(&expr, "x", 1.0);
        assert!(result.to_string().contains("sin"), "sin(y) 不应改变: {}", result);
    }

    // ─────────────────────────────────────────────
    // 2. substitute — 符号替换
    // ─────────────────────────────────────────────

    #[test]
    fn test_substitute_symbol_to_expr() {
        // (+ x y), x => (sin z), y => 1  =>  (+ (sin z) 1)
        let expr: RecExpr<MathExpr> = "(+ x y)".parse().unwrap();
        let a:    RecExpr<MathExpr> = "(sin z)".parse().unwrap();
        let one:  RecExpr<MathExpr> = "1".parse().unwrap();
        let result = substitute(&expr, &[("x", a), ("y", one)]);
        println!("substitute((+ x y)) = {}", result);
        assert!(result.to_string().contains("sin"), "应含 sin: {}", result);
    }

    #[test]
    fn test_substitute_chain() {
        // x^2 with x => (+ a b)  =>  (+ a b)^2
        let expr: RecExpr<MathExpr> = "(pow x 2)".parse().unwrap();
        let ab:   RecExpr<MathExpr> = "(+ a b)".parse().unwrap();
        let result = substitute(&expr, &[("x", ab)]);
        println!("(x^2)[x := a+b] = {}", result);
        // Not simplified symbolically, but should contain pow
        assert!(result.to_string().contains("pow"), "应含 pow: {}", result);
    }

    #[test]
    fn test_substitute_no_match() {
        // sin(y) with x => 0 — y 不受影响
        let expr: RecExpr<MathExpr> = "(sin y)".parse().unwrap();
        let zero:  RecExpr<MathExpr> = "0".parse().unwrap();
        let result = substitute(&expr, &[("x", zero)]);
        assert_eq!(result.to_string(), "(sin y)");
    }

    // ─────────────────────────────────────────────
    // 3. expand — 代数展开
    // ─────────────────────────────────────────────

    #[test]
    fn test_expand_left_distribute() {
        // x * (y + z) => x*y + x*z
        let expr: RecExpr<MathExpr> = "(* x (+ y z))".parse().unwrap();
        let expanded = expand(&expr);
        println!("expand(x*(y+z)) = {}", expanded);
        let s = expanded.to_string();
        // Should contain both mul and add somewhere in the tree
        assert!(s.contains('+'), "应含加号: {}", s);
        // After simplify, it may be refactored. Verify semantically:
        let f = build_function(&expanded, &["x", "y", "z"]);
        let f_orig = build_function(&expr, &["x", "y", "z"]);
        for (x, y, z) in [(1.0, 2.0, 3.0), (-1.0, 5.0, 0.5)] {
            assert!((f(&[x, y, z]) - f_orig(&[x, y, z])).abs() < 1e-10,
                "expand result differs at ({},{},{})", x, y, z);
        }
    }

    #[test]
    fn test_expand_right_distribute() {
        // (x + y) * z => x*z + y*z
        let expr: RecExpr<MathExpr> = "(* (+ x y) z)".parse().unwrap();
        let expanded = expand(&expr);
        println!("expand((x+y)*z) = {}", expanded);
        let s = expanded.to_string();
        // Verify semantically that expansion preserves value
        let f = build_function(&expanded, &["x", "y", "z"]);
        let f_orig = build_function(&expr, &["x", "y", "z"]);
        assert!((f(&[2.0, 3.0, 5.0]) - f_orig(&[2.0, 3.0, 5.0])).abs() < 1e-10);
        assert!(s.contains('+'), "应含加号: {}", s);
    }

    #[test]
    fn test_expand_double_distribute() {
        // (x + y) * (a + b) => x*a + x*b + y*a + y*b
        let expr: RecExpr<MathExpr> = "(* (+ x y) (+ a b))".parse().unwrap();
        let expanded = expand(&expr);
        println!("expand((x+y)*(a+b)) = {}", expanded);
        let s = expanded.to_string();
        // Verify semantically that (x+y)*(a+b) expand equals original
        let f = build_function(&expanded, &["x", "y", "a", "b"]);
        let f_orig = build_function(&expr, &["x", "y", "a", "b"]);
        assert!((f(&[2.0, 3.0, 4.0, 5.0]) - f_orig(&[2.0, 3.0, 4.0, 5.0])).abs() < 1e-10,
            "expand((x+y)*(a+b)) = {}", s);
    }

    #[test]
    fn test_expand_constant_fold() {
        // 2 * (3 + x) => 6 + 2*x (常量部分折叠)
        let expr: RecExpr<MathExpr> = "(* 2 (+ 3 x))".parse().unwrap();
        let expanded = expand(&expr);
        println!("expand(2*(3+x)) = {}", expanded);
        let s = expanded.to_string();
        // 6 + 2*x 或等价形式
        assert!(s.contains('+'), "应含加号: {}", s);
    }

    #[test]
    fn test_expand_with_subtraction() {
        // x * (y - z) => x*y - x*z
        let expr: RecExpr<MathExpr> = "(* x (- y z))".parse().unwrap();
        let expanded = expand(&expr);
        println!("expand(x*(y-z)) = {}", expanded);
        // Should expand, not be a single Mul
        let s = expanded.to_string();
        assert!(!s.starts_with("(* x"), "顶层不应是 x*(y-z): {}", s);
    }

    // ─────────────────────────────────────────────
    // 4. degree — 多项式次数
    // ─────────────────────────────────────────────

    #[test]
    fn test_degree_constant() {
        let expr: RecExpr<MathExpr> = "5".parse().unwrap();
        assert_eq!(degree(&expr, "x"), 0);
    }

    #[test]
    fn test_degree_linear() {
        let expr: RecExpr<MathExpr> = "(+ (* 3 x) 1)".parse().unwrap();
        assert_eq!(degree(&expr, "x"), 1);
        assert_eq!(degree(&expr, "y"), 0); // y 不存在
    }

    #[test]
    fn test_degree_quadratic() {
        // x^2 + 2x + 1
        let expr: RecExpr<MathExpr> = "(+ (+ (pow x 2) (* 2 x)) 1)".parse().unwrap();
        assert_eq!(degree(&expr, "x"), 2);
    }

    #[test]
    fn test_degree_cubic() {
        // x^3
        let expr: RecExpr<MathExpr> = "(pow x 3)".parse().unwrap();
        assert_eq!(degree(&expr, "x"), 3);
    }

    #[test]
    fn test_degree_multivariate() {
        // x^2 * y^3 — degree in x=2, in y=3
        let expr: RecExpr<MathExpr> = "(* (pow x 2) (pow y 3))".parse().unwrap();
        assert_eq!(degree(&expr, "x"), 2);
        assert_eq!(degree(&expr, "y"), 3);
    }

    #[test]
    fn test_degree_transcendental() {
        // sin(x) — 超越函数，非多项式
        let expr: RecExpr<MathExpr> = "(sin x)".parse().unwrap();
        assert_eq!(degree(&expr, "x"), i32::MAX);
    }

    #[test]
    fn test_degree_rational() {
        // x / 2 — 次数 1（分母为常数）
        let expr: RecExpr<MathExpr> = "(/ x 2)".parse().unwrap();
        assert_eq!(degree(&expr, "x"), 1);
    }

    // ─────────────────────────────────────────────
    // 5. is_linear / is_affine
    // ─────────────────────────────────────────────

    #[test]
    fn test_is_linear() {
        let linear: RecExpr<MathExpr> = "(+ (* 3 x) 5)".parse().unwrap();
        assert!(is_linear(&linear, "x"), "3x+5 should be linear in x");

        let quad: RecExpr<MathExpr> = "(pow x 2)".parse().unwrap();
        assert!(!is_linear(&quad, "x"), "x^2 should NOT be linear in x");

        let trig: RecExpr<MathExpr> = "(sin x)".parse().unwrap();
        assert!(!is_linear(&trig, "x"), "sin(x) should NOT be linear in x");

        // Constant w.r.t. y
        let expr: RecExpr<MathExpr> = "(+ (* 3 x) 5)".parse().unwrap();
        assert!(is_linear(&expr, "y"), "3x+5 is trivially linear in y (degree 0)");
    }

    #[test]
    fn test_is_affine() {
        let expr: RecExpr<MathExpr> = "(+ (+ (* 2 x) (* 3 y)) 1)".parse().unwrap();
        assert!(is_affine(&expr, &["x", "y"]), "2x + 3y + 1 should be affine");

        let nonlinear: RecExpr<MathExpr> = "(+ (pow x 2) y)".parse().unwrap();
        assert!(!is_affine(&nonlinear, &["x", "y"]), "x^2 + y is NOT affine");
    }

    // ─────────────────────────────────────────────
    // 6. build_function — 数值求值
    // ─────────────────────────────────────────────

    #[test]
    fn test_build_function_linear() {
        // f(x) = 2x + 1
        let expr: RecExpr<MathExpr> = "(+ (* 2 x) 1)".parse().unwrap();
        let f = build_function(&expr, &["x"]);
        assert!((f(&[3.0]) - 7.0).abs() < 1e-10, "f(3) = {}", f(&[3.0]));
        assert!((f(&[0.0]) - 1.0).abs() < 1e-10, "f(0) = {}", f(&[0.0]));
        assert!((f(&[-1.0]) - (-1.0)).abs() < 1e-10, "f(-1) = {}", f(&[-1.0]));
    }

    #[test]
    fn test_build_function_quadratic() {
        // f(x) = x^2 + x + 1
        let expr: RecExpr<MathExpr> = "(+ (+ (pow x 2) x) 1)".parse().unwrap();
        let f = build_function(&expr, &["x"]);
        assert!((f(&[2.0]) - 7.0).abs() < 1e-10);
        assert!((f(&[0.0]) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_build_function_trig() {
        // f(x) = sin(x)^2 + cos(x)^2 = 1
        let expr: RecExpr<MathExpr> = "(+ (pow (sin x) 2) (pow (cos x) 2))".parse().unwrap();
        let f = build_function(&expr, &["x"]);
        for &v in &[0.0, 1.0, 2.0, -1.0, std::f64::consts::PI] {
            assert!((f(&[v]) - 1.0).abs() < 1e-10, "sin²({})+cos²({}) = {}", v, v, f(&[v]));
        }
    }

    #[test]
    fn test_build_function_multivariate() {
        // f(x, y) = x^2 + y^2
        let expr: RecExpr<MathExpr> = "(+ (pow x 2) (pow y 2))".parse().unwrap();
        let f = build_function(&expr, &["x", "y"]);
        assert!((f(&[3.0, 4.0]) - 25.0).abs() < 1e-10, "3²+4² = {}", f(&[3.0, 4.0]));
    }

    #[test]
    fn test_build_function_sign_floor_ceil() {
        let sign_expr: RecExpr<MathExpr> = "(sign x)".parse().unwrap();
        let floor_expr: RecExpr<MathExpr> = "(floor x)".parse().unwrap();
        let ceil_expr: RecExpr<MathExpr> = "(ceil x)".parse().unwrap();

        let fsign  = build_function(&sign_expr, &["x"]);
        let ffloor = build_function(&floor_expr, &["x"]);
        let fceil  = build_function(&ceil_expr, &["x"]);

        assert_eq!(fsign(&[-3.0]), -1.0);
        assert_eq!(fsign(&[5.0]),   1.0);
        assert_eq!(ffloor(&[2.7]),  2.0);
        assert_eq!(ffloor(&[-2.7]), -3.0);
        assert_eq!(fceil(&[2.1]),   3.0);
        assert_eq!(fceil(&[-2.9]), -2.0);
    }

    // ─────────────────────────────────────────────
    // 7. hessian — 黑塞矩阵
    // ─────────────────────────────────────────────

    #[test]
    fn test_hessian_quadratic_diagonal() {
        // f(x, y) = x^2 + y^2  =>  H = [[2, 0], [0, 2]]
        let f: RecExpr<MathExpr> = "(+ (* x x) (* y y))".parse().unwrap();
        let h = hessian(&f, &["x", "y"]);

        // Hessian diagonal test — verify via build_function evaluation
        // For f = x^2 + y^2, H = [[2, 0], [0, 2]]
        // H[0][0] = ∂²f/∂x² = 2: evaluate at (x,y) = (1,1)
        let h00_f = build_function(&h[0][0], &["x", "y"]);
        let h11_f = build_function(&h[1][1], &["x", "y"]);
        let h01_f = build_function(&h[0][1], &["x", "y"]);
        let h10_f = build_function(&h[1][0], &["x", "y"]);
        println!("H[0][0] at (1,1) = {}", h00_f(&[1.0, 1.0]));
        println!("H[1][1] at (1,1) = {}", h11_f(&[1.0, 1.0]));
        println!("H[0][1] at (1,1) = {}", h01_f(&[1.0, 1.0]));
        assert!((h00_f(&[1.0, 1.0]) - 2.0).abs() < 1e-6, "H[0][0] = {}", h00_f(&[1.0, 1.0]));
        assert!((h11_f(&[1.0, 1.0]) - 2.0).abs() < 1e-6, "H[1][1] = {}", h11_f(&[1.0, 1.0]));
        assert!((h01_f(&[1.0, 1.0])).abs() < 1e-6, "H[0][1] = {}", h01_f(&[1.0, 1.0]));
        assert!((h10_f(&[1.0, 1.0])).abs() < 1e-6, "H[1][0] = {}", h10_f(&[1.0, 1.0]));
    }

    #[test]
    fn test_hessian_xy_cross_term() {
        // f(x, y) = x * y  =>  H = [[0, 1], [1, 0]]
        let f: RecExpr<MathExpr> = "(* x y)".parse().unwrap();
        let h = hessian(&f, &["x", "y"]);

        // H[0][0] = 0, H[0][1] = 1, H[1][0] = 1, H[1][1] = 0
        let h01_f = build_function(&h[0][1], &["x", "y"]);
        let h10_f = build_function(&h[1][0], &["x", "y"]);
        let h00_f = build_function(&h[0][0], &["x", "y"]);
        println!("H[0][1] at (1,2) = {}, H[1][0] at (1,2) = {}", h01_f(&[1.0, 2.0]), h10_f(&[1.0, 2.0]));
        // 交叉项应为非零（系数等于 1），在任意点求值应等于 1
        assert!((h01_f(&[1.0, 2.0]) - 1.0).abs() < 1e-6, "H[0][1] should eval to 1");
        assert!((h10_f(&[1.0, 2.0]) - 1.0).abs() < 1e-6, "H[1][0] should eval to 1");
        assert!((h00_f(&[1.0, 2.0])).abs() < 1e-6, "H[0][0] should be 0");
    }

    // ─────────────────────────────────────────────
    // 8. taylor_coeff + taylor — 泰勒展开
    // ─────────────────────────────────────────────

    #[test]
    fn test_taylor_coeff_exp_at_0() {
        // exp(x) at x=0: all coefficients are 1/n!
        let f: RecExpr<MathExpr> = "(exp x)".parse().unwrap();

        let c0 = taylor_coeff(&f, "x", 0);
        let c1 = taylor_coeff(&f, "x", 1);
        let c2 = taylor_coeff(&f, "x", 2);
        let c3 = taylor_coeff(&f, "x", 3);

        // Verify via build_function at x=0 for each coefficient
        let c0_f = build_function(&c0, &["x"]);
        let c1_f = build_function(&c1, &["x"]);
        let c2_f = build_function(&c2, &["x"]);
        let c3_f = build_function(&c3, &["x"]);
        println!("c0={}, c1={}, c2={}, c3={}", c0_f(&[0.0]), c1_f(&[0.0]), c2_f(&[0.0]), c3_f(&[0.0]));
        assert!((c0_f(&[0.0]) - 1.0).abs() < 1e-6, "c0 of exp(x) = 1, got {}", c0_f(&[0.0]));
        assert!((c1_f(&[0.0]) - 1.0).abs() < 1e-6, "c1 of exp(x) = 1, got {}", c1_f(&[0.0]));
        assert!((c2_f(&[0.0]) - 0.5).abs() < 1e-6, "c2 = 0.5, got {}", c2_f(&[0.0]));
        assert!((c3_f(&[0.0]) - 1.0/6.0).abs() < 1e-6, "c3 = 1/6, got {}", c3_f(&[0.0]));
    }

    #[test]
    fn test_taylor_coeff_sin_at_0() {
        // sin(x) at x=0: c0=0, c1=1, c2=0, c3=-1/6
        let f: RecExpr<MathExpr> = "(sin x)".parse().unwrap();

        let c0 = taylor_coeff(&f, "x", 0);
        let c1 = taylor_coeff(&f, "x", 1);
        let c2 = taylor_coeff(&f, "x", 2);
        let c3 = taylor_coeff(&f, "x", 3);

        // Verify sin taylor coefficients via build_function at x=0
        let c0_f = build_function(&c0, &["x"]);
        let c1_f = build_function(&c1, &["x"]);
        let c2_f = build_function(&c2, &["x"]);
        let c3_f = build_function(&c3, &["x"]);
        println!("sin c0={}, c1={}, c2={}, c3={}", c0_f(&[0.0]), c1_f(&[0.0]), c2_f(&[0.0]), c3_f(&[0.0]));
        assert!((c0_f(&[0.0]) - 0.0).abs() < 1e-6, "sin c0 = 0");
        assert!((c1_f(&[0.0]) - 1.0).abs() < 1e-6, "sin c1 = 1");
        assert!((c2_f(&[0.0]) - 0.0).abs() < 1e-6, "sin c2 = 0");
        assert!((c3_f(&[0.0]) + 1.0/6.0).abs() < 1e-6, "sin c3 = -1/6");
    }

    #[test]
    fn test_taylor_polynomial_sum() {
        // taylor(exp(x), x, 3) should be ≈ 1 + x + 0.5*x^2 + (1/6)*x^3
        let f: RecExpr<MathExpr> = "(exp x)".parse().unwrap();
        let t3 = taylor(&f, "x", 3);
        println!("taylor(exp(x), x, 3) = {}", t3);

        // Evaluate numerically to check: at x=0.1
        let eval = build_function(&t3, &["x"]);
        let exact = 0.1_f64.exp();
        let approx = eval(&[0.1]);
        println!("taylor approx at x=0.1: {}, exact: {}", approx, exact);
        assert!((approx - exact).abs() < 0.001, "Taylor approx error too large: {}", (approx - exact).abs());
    }

    #[test]
    fn test_taylor_sin_3rd_order() {
        // taylor(sin(x), x, 3) ≈ x - x^3/6
        let f: RecExpr<MathExpr> = "(sin x)".parse().unwrap();
        let t3 = taylor(&f, "x", 3);
        println!("taylor(sin(x), x, 3) = {}", t3);

        let eval = build_function(&t3, &["x"]);
        let x = 0.2_f64;
        let approx = eval(&[x]);
        let exact = x.sin();
        assert!((approx - exact).abs() < 0.001, "|approx - exact| = {}", (approx - exact).abs());
    }

    // ─────────────────────────────────────────────
    // 9. coeff_of — 系数提取
    // ─────────────────────────────────────────────

    #[test]
    fn test_coeff_of_linear() {
        // 3x + 2y: coeff of x = 3, coeff of y = 2
        let expr: RecExpr<MathExpr> = "(+ (* 3 x) (* 2 y))".parse().unwrap();

        let cx = coeff_of(&expr, "x");
        let cy = coeff_of(&expr, "y");
        println!("coeff of x: {}, coeff of y: {}", cx, cy);

        // coeff_of uses derivative trick: f'(0). Check numerically.
        let cx_f = build_function(&cx, &["x", "y"]);
        let cy_f = build_function(&cy, &["x", "y"]);
        assert!((cx_f(&[0.0, 0.0]) - 3.0).abs() < 1e-6, "coeff of x = 3, got {}", cx_f(&[0.0, 0.0]));
        assert!((cy_f(&[0.0, 0.0]) - 2.0).abs() < 1e-6, "coeff of y = 2, got {}", cy_f(&[0.0, 0.0]));
    }

    #[test]
    fn test_coeff_of_n_quadratic() {
        // x^2 + 2x + 1: coeff of x^2 = 1, coeff of x^1 = 2, coeff of x^0 = 1
        let expr: RecExpr<MathExpr> = "(+ (+ (pow x 2) (* 2 x)) 1)".parse().unwrap();

        let c2 = coeff_of_n(&expr, "x", 2);
        let c1 = coeff_of_n(&expr, "x", 1);
        let c0 = coeff_of_n(&expr, "x", 0);
        println!("x^2 coeff:{} x^1 coeff:{} x^0 coeff:{}", c2, c1, c0);

        // Verify via build_function at x=0: each coeff can be tested numerically
        // c0 = f(0) = 1, c1 = f'(0) = 2, c2 = f''(0)/2 = 1
        let c0_f = build_function(&c0, &["x"]);
        let c1_f = build_function(&c1, &["x"]);
        let c2_f = build_function(&c2, &["x"]);
        println!("c0={}, c1={}, c2={}", c0_f(&[0.0]), c1_f(&[0.0]), c2_f(&[0.0]));
        assert!((c0_f(&[0.0]) - 1.0).abs() < 1e-6, "coeff x^0 = 1, got {}", c0_f(&[0.0]));
        assert!((c1_f(&[0.0]) - 2.0).abs() < 1e-6, "coeff x^1 = 2, got {}", c1_f(&[0.0]));
        assert!((c2_f(&[0.0]) - 1.0).abs() < 1e-6, "coeff x^2 = 1, got {}", c2_f(&[0.0]));
    }

    // ─────────────────────────────────────────────
    // 10. Equation — 符号方程式
    // ─────────────────────────────────────────────

    #[test]
    fn test_equation_display() {
        let eq = Equation::parse("(pow x 2)", "1");
        println!("Equation: {}", eq);
        let s = eq.to_string();
        assert!(s.contains('~'), "Display should contain ~: {}", s);
    }

    #[test]
    fn test_equation_canonical() {
        // x^2 ~ 1  =>  canonical: x^2 - 1
        let eq = Equation::parse("(pow x 2)", "1");
        let canonical = eq.to_canonical();
        println!("canonical(x^2 ~ 1) = {}", canonical);
        // At x=1: 1-1=0, at x=2: 4-1=3
        let f = build_function(&canonical, &["x"]);
        assert!((f(&[1.0]) - 0.0).abs() < 1e-10, "canonical at x=1 should be 0");
        assert!((f(&[2.0]) - 3.0).abs() < 1e-10, "canonical at x=2 should be 3");
    }

    #[test]
    fn test_equation_evaluate() {
        // x^2 ~ 1, at x=1 => true, at x=2 => false
        let eq = Equation::parse("(pow x 2)", "1");
        assert!(eq.evaluate(&[("x", 1.0)]),  "x=1: 1 ~ 1 should be true");
        assert!(eq.evaluate(&[("x", -1.0)]), "x=-1: 1 ~ 1 should be true");
        assert!(!eq.evaluate(&[("x", 2.0)]), "x=2: 4 ~ 1 should be false");
    }

    #[test]
    fn test_equation_simplify_both() {
        // (+ x x) ~ (* 2 x)  =>  simplified lhs: 2x, rhs: 2x
        let lhs: RecExpr<MathExpr> = "(+ x x)".parse().unwrap();
        let rhs: RecExpr<MathExpr> = "(* 2 x)".parse().unwrap();
        let eq = Equation::new(lhs, rhs).simplify_both();
        println!("simplified eq: {}", eq);
        // After simplification, both sides represent the same function (2x)
        // Evaluate at x=3: lhs should equal rhs
        let fl = build_function(&eq.lhs, &["x"]);
        let fr = build_function(&eq.rhs, &["x"]);
        assert!((fl(&[3.0]) - fr(&[3.0])).abs() < 1e-10,
            "both sides evaluated at x=3 should be equal: {} vs {}", fl(&[3.0]), fr(&[3.0]));
    }

    // ─────────────────────────────────────────────
    // 11. LaTeX — Sign/Floor/Ceil 渲染
    // ─────────────────────────────────────────────

    #[test]
    fn test_latex_sign() {
        let e: RecExpr<MathExpr> = "(sign x)".parse().unwrap();
        let tex = to_latex(&e);
        println!("LaTeX sign: {}", tex);
        assert_eq!(tex, r"\operatorname{sgn}\left(x\right)");
    }

    #[test]
    fn test_latex_floor() {
        let e: RecExpr<MathExpr> = "(floor x)".parse().unwrap();
        let tex = to_latex(&e);
        println!("LaTeX floor: {}", tex);
        assert_eq!(tex, r"\left\lfloor x\right\rfloor");
    }

    #[test]
    fn test_latex_ceil() {
        let e: RecExpr<MathExpr> = "(ceil x)".parse().unwrap();
        let tex = to_latex(&e);
        println!("LaTeX ceil: {}", tex);
        assert_eq!(tex, r"\left\lceil x\right\rceil");
    }

    // ─────────────────────────────────────────────
    // 12. 常量折叠 — Sign/Floor/Ceil 数值化简
    // ─────────────────────────────────────────────

    #[test]
    fn test_constant_folding_sign() {
        // sign is a piecewise function; test via build_function
        // Note: Rust's f64::signum(0.0) = 1.0 (positive zero = positive)
        let f_sign = build_function(&"(sign x)".parse().unwrap(), &["x"]);
        assert_eq!(f_sign(&[-3.0]), -1.0);
        assert_eq!(f_sign(&[5.0]),   1.0);
        assert_eq!(f_sign(&[0.0]),   1.0); // Rust: 0_f64.signum() = 1.0
    }

    #[test]
    fn test_constant_folding_floor_ceil() {
        // Verify floor/ceil evaluation via build_function interpreter
        let f_floor = build_function(&"(floor x)".parse().unwrap(), &["x"]);
        let f_ceil  = build_function(&"(ceil x)".parse().unwrap(), &["x"]);
        assert_eq!(f_floor(&[2.7]),   2.0);
        assert_eq!(f_floor(&[-2.7]), -3.0);
        assert_eq!(f_ceil(&[2.1]),    3.0);
        assert_eq!(f_ceil(&[-2.9]),  -2.0);
    }
}
