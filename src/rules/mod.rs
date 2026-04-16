use crate::expr::{MathAnalysis, MathExpr};
use egg::{rewrite as rw, Rewrite, Language, RecExpr, Runner, Extractor};

pub fn calculus_and_algebra_rules() -> Vec<Rewrite<MathExpr, MathAnalysis>> {
    vec![
        // ==================
        // 1. 基础代数化简 (Algebra)
        // ==================
        rw!("add-comm"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rw!("mul-comm"; "(* ?a ?b)" => "(* ?b ?a)"),
        rw!("add-zero"; "(+ ?a 0)"  => "?a"),
        rw!("sub-zero"; "(- ?a 0)"  => "?a"),
        rw!("mul-one";  "(* ?a 1)"  => "?a"),
        rw!("mul-zero"; "(* ?a 0)"  => "0"),
        rw!("div-one";  "(/ ?a 1)"  => "?a"),
        // 数值常量的除法化简
        rw!("div-self-num"; "(/ ?a ?a)" => "1" if is_not_zero("?a")),
        // 符号原子变量的除法化简 (参考 Symbolics.jl：CAS 通常假设符号变量非零)
        rw!("div-self-sym"; "(/ ?a ?a)" => "1" if is_symbol_atom("?a")),
        rw!("sub-self"; "(- ?a ?a)" => "0"),
        rw!("neg-neg";  "(neg (neg ?a))" => "?a"),
        rw!("pow-zero"; "(pow ?a 0)" => "1" if is_not_zero("?a")),
        rw!("pow-one";  "(pow ?a 1)" => "?a"),
        rw!("sqrt-pow"; "(sqrt ?a)" => "(pow ?a 0.5)"),

        // 合并同类项与因式分解 (Combine like terms & Factorization)
        rw!("add-same"; "(+ ?a ?a)" => "(* 2 ?a)"),
        rw!("sub-same"; "(- ?a ?a)" => "0"),
        rw!("mul-same"; "(* ?a ?a)" => "(pow ?a 2)"),
        rw!("distribute";   "(* ?a (+ ?b ?c))" => "(+ (* ?a ?b) (* ?a ?c))"),
        rw!("factor"; "(+ (* ?a ?b) (* ?a ?c))" => "(* ?a (+ ?b ?c))"),
        // 数值常量的乘除消除
        rw!("cancel-div-mul";   "(/ (* ?a ?b) ?a)" => "?b" if is_not_zero("?a")),
        rw!("cancel-div-mul-2"; "(/ (* ?b ?a) ?a)" => "?b" if is_not_zero("?a")),
        // 符号原子变量的乘除消除
        rw!("cancel-div-mul-sym";   "(/ (* ?a ?b) ?a)" => "?b" if is_symbol_atom("?a")),
        rw!("cancel-div-mul-2-sym"; "(/ (* ?b ?a) ?a)" => "?b" if is_symbol_atom("?a")),

        // 指数与对数化简 (Log & Exp Rules)
        rw!("log-exp"; "(log (exp ?a))" => "?a"),
        rw!("exp-log"; "(exp (log ?a))" => "?a"),
        rw!("log-mul"; "(log (* ?a ?b))" => "(+ (log ?a) (log ?b))"),
        rw!("log-pow"; "(log (pow ?a ?b))" => "(* ?b (log ?a))"),
        rw!("pow-mul"; "(* (pow ?a ?b) (pow ?a ?c))" => "(pow ?a (+ ?b ?c))"),

        // ==================
        // 2. 三角函数恒等式 (Trigonometry)
        // ==================
        // 奇偶性
        rw!("sin-neg"; "(sin (neg ?a))" => "(neg (sin ?a))"),
        rw!("cos-neg"; "(cos (neg ?a))" => "(cos ?a)"),
        rw!("tan-neg"; "(tan (neg ?a))" => "(neg (tan ?a))"),
        // 毕达哥拉斯三角恒等式: sin^2(x) + cos^2(x) = 1
        rw!("sin-cos-sq"; "(+ (pow (sin ?a) 2) (pow (cos ?a) 2))" => "1"),
        rw!("cos-sin-sq"; "(+ (pow (cos ?a) 2) (pow (sin ?a) 2))" => "1"),
        // 定义展开 (用于化简求导后的结果)
        rw!("tan-def"; "(tan ?a)"  => "(/ (sin ?a) (cos ?a))"),
        rw!("sec-def"; "(sec ?a)"  => "(/ 1 (cos ?a))"),
        rw!("csc-def"; "(csc ?a)"  => "(/ 1 (sin ?a))"),
        rw!("cot-def"; "(cot ?a)"  => "(/ (cos ?a) (sin ?a))"),
        rw!("log2-def"; "(log2 ?a)" => "(/ (log ?a) (log 2))"),

        // ==================
        // 3. 双曲函数恒等式 (Hyperbolic)
        // ==================
        rw!("sinh-neg"; "(sinh (neg ?a))" => "(neg (sinh ?a))"),
        rw!("cosh-neg"; "(cosh (neg ?a))" => "(cosh ?a)"),
        rw!("tanh-neg"; "(tanh (neg ?a))" => "(neg (tanh ?a))"),
        // 双曲基本恒等式: cosh^2(x) - sinh^2(x) = 1
        rw!("cosh-sinh-sq"; "(- (pow (cosh ?a) 2) (pow (sinh ?a) 2))" => "1"),
        rw!("tanh-def"; "(tanh ?a)" => "(/ (sinh ?a) (cosh ?a))"),

        // ==================
        // 4. 微积分与求导法则 (Calculus)
        // ==================
        // 常数求导为 0
        rw!("diff-const"; "(diff ?c ?x)" => "0" if is_const_num("?c")),
        rw!("diff-var";   "(diff ?x ?x)" => "1"),
        // 对不同变量的求导（偏导默认互相独立，当做常数）
        rw!("diff-diff-var"; "(diff ?y ?x)" => "0" if is_diff_var("?y", "?x")),

        // 基本线性法则
        rw!("diff-neg";  "(diff (neg ?a) ?x)" => "(neg (diff ?a ?x))"),
        rw!("diff-add";  "(diff (+ ?a ?b) ?x)" => "(+ (diff ?a ?x) (diff ?b ?x))"),
        rw!("diff-sub";  "(diff (- ?a ?b) ?x)" => "(- (diff ?a ?x) (diff ?b ?x))"),

        // 乘法法则 (u v)' = u' v + u v'
        rw!("diff-mul"; "(diff (* ?a ?b) ?x)" => "(+ (* (diff ?a ?x) ?b) (* ?a (diff ?b ?x)))"),
        // 除法法则 (u / v)' = (u' v - u v') / v^2
        rw!("diff-div"; "(diff (/ ?a ?b) ?x)" => "(/ (- (* (diff ?a ?x) ?b) (* ?a (diff ?b ?x))) (pow ?b 2))"),

        // 幂法则: x^n -> n * x^(n-1) * dx/dx  (当 n 为常数时)
        rw!("diff-pow-const"; "(diff (pow ?a ?n) ?x)" => "(* (* ?n (pow ?a (- ?n 1))) (diff ?a ?x))" if is_const_num("?n")),
        // 通用幂法则: (u^v)' = u^v * (v' * ln(u) + v * u' / u)
        // 注：仅在指数非常数时触发（常数指数由 diff-pow-const 处理）
        rw!("diff-pow-var"; "(diff (pow ?u ?v) ?x)" => "(* (pow ?u ?v) (+ (* (diff ?v ?x) (log ?u)) (* ?v (/ (diff ?u ?x) ?u))))" if is_not_const_num("?v")),
        rw!("diff-sqrt"; "(diff (sqrt ?a) ?x)" => "(/ (diff ?a ?x) (* 2 (sqrt ?a)))"),

        // 三角函数求导 (参考 Symbolics.jl register_derivatives.jl)
        rw!("diff-sin";  "(diff (sin ?a) ?x)" => "(* (cos ?a) (diff ?a ?x))"),
        rw!("diff-cos";  "(diff (cos ?a) ?x)" => "(* (neg (sin ?a)) (diff ?a ?x))"),
        rw!("diff-tan";  "(diff (tan ?a) ?x)" => "(/ (diff ?a ?x) (pow (cos ?a) 2))"),
        rw!("diff-asin"; "(diff (asin ?a) ?x)" => "(/ (diff ?a ?x) (sqrt (- 1 (pow ?a 2))))"),
        rw!("diff-acos"; "(diff (acos ?a) ?x)" => "(/ (neg (diff ?a ?x)) (sqrt (- 1 (pow ?a 2))))"),
        rw!("diff-atan"; "(diff (atan ?a) ?x)" => "(/ (diff ?a ?x) (+ 1 (pow ?a 2)))"),
        // d/dx sec(u) = sec(u)*tan(u) * u'
        rw!("diff-sec";  "(diff (sec ?a) ?x)" => "(* (* (sec ?a) (tan ?a)) (diff ?a ?x))"),
        // d/dx csc(u) = -csc(u)*cot(u) * u'
        rw!("diff-csc";  "(diff (csc ?a) ?x)" => "(* (neg (* (csc ?a) (cot ?a))) (diff ?a ?x))"),
        // d/dx cot(u) = -csc(u)^2 * u'
        rw!("diff-cot";  "(diff (cot ?a) ?x)" => "(* (neg (pow (csc ?a) 2)) (diff ?a ?x))"),
        // d/dx asec(u) = u' / (|u| * sqrt(u^2 - 1))
        rw!("diff-asec"; "(diff (asec ?a) ?x)" => "(/ (diff ?a ?x) (* (abs ?a) (sqrt (- (pow ?a 2) 1))))"),
        // d/dx acsc(u) = -u' / (|u| * sqrt(u^2 - 1))
        rw!("diff-acsc"; "(diff (acsc ?a) ?x)" => "(/ (neg (diff ?a ?x)) (* (abs ?a) (sqrt (- (pow ?a 2) 1))))"),
        // d/dx acot(u) = -u' / (1 + u^2)
        rw!("diff-acot"; "(diff (acot ?a) ?x)" => "(/ (neg (diff ?a ?x)) (+ 1 (pow ?a 2)))"),

        // 双曲函数求导
        rw!("diff-sinh";  "(diff (sinh ?a) ?x)"  => "(* (cosh ?a) (diff ?a ?x))"),
        rw!("diff-cosh";  "(diff (cosh ?a) ?x)"  => "(* (sinh ?a) (diff ?a ?x))"),
        rw!("diff-tanh";  "(diff (tanh ?a) ?x)"  => "(/ (diff ?a ?x) (pow (cosh ?a) 2))"),
        rw!("diff-asinh"; "(diff (asinh ?a) ?x)" => "(/ (diff ?a ?x) (sqrt (+ (pow ?a 2) 1)))"),
        rw!("diff-acosh"; "(diff (acosh ?a) ?x)" => "(/ (diff ?a ?x) (sqrt (- (pow ?a 2) 1)))"),
        rw!("diff-atanh"; "(diff (atanh ?a) ?x)" => "(/ (diff ?a ?x) (- 1 (pow ?a 2)))"),

        // 指数与对数求导
        rw!("diff-exp";  "(diff (exp ?a) ?x)"  => "(* (exp ?a) (diff ?a ?x))"),
        rw!("diff-log";  "(diff (log ?a) ?x)"  => "(/ (diff ?a ?x) ?a)"),
        // d/dx log2(u) = u' / (u * ln(2)) — 参考 Symbolics.jl DiffRules 生成的规则
        rw!("diff-log2"; "(diff (log2 ?a) ?x)" => "(/ (diff ?a ?x) (* ?a (log 2)))"),

        // abs 求导（分段，与 Symbolics.jl 保持一致用 neg 表示负侧）
        rw!("diff-abs"; "(diff (abs ?a) ?x)" => "(* (/ ?a (abs ?a)) (diff ?a ?x))"),

        // sign/floor/ceil 求导均为 0（分段常数函数，参考 Symbolics.jl extra_functions.jl）
        rw!("diff-sign";  "(diff (sign ?a) ?x)"  => "0"),
        rw!("diff-floor"; "(diff (floor ?a) ?x)" => "0"),
        rw!("diff-ceil";  "(diff (ceil ?a) ?x)"  => "0"),
    ]
}


/// 判断 var1 是否是和 var2 不同的独立符号变量
fn is_diff_var(var1: &'static str, var2: &'static str) -> impl Fn(&mut egg::EGraph<MathExpr, MathAnalysis>, egg::Id, &egg::Subst) -> bool {
    let v1: egg::Var = var1.parse().unwrap();
    let v2: egg::Var = var2.parse().unwrap();
    move |egraph, _, subst| {
        let id1 = subst[v1];
        let id2 = subst[v2];

        let mut sym1 = None;
        for node in egraph[id1].nodes.iter() {
            if let MathExpr::Symbol(s) = node {
                sym1 = Some(*s);
                break;
            }
        }

        if let Some(s1) = sym1 {
            for node in egraph[id2].nodes.iter() {
                if let MathExpr::Symbol(s2) = node {
                    if s1 == *s2 {
                        return false;
                    }
                }
            }
            return true;
        }
        false
    }
}

/// 判断某节点是否为确切的常数（完全不包含未赋值的变量符号）
fn is_const_num(var: &'static str) -> impl Fn(&mut egg::EGraph<MathExpr, MathAnalysis>, egg::Id, &egg::Subst) -> bool {
    let var_sym: egg::Var = var.parse().unwrap();
    move |egraph, _, subst| {
        let id = subst[var_sym];
        // 如果 `data` 有值，代表常量折叠引擎成功把它算成了一个数字
        egraph[id].data.is_some()
    }
}

/// `is_const_num` 的补集 — 节点不是常数（含有符号变量）
fn is_not_const_num(var: &'static str) -> impl Fn(&mut egg::EGraph<MathExpr, MathAnalysis>, egg::Id, &egg::Subst) -> bool {
    let var_sym: egg::Var = var.parse().unwrap();
    move |egraph, _, subst| {
        let id = subst[var_sym];
        egraph[id].data.is_none()
    }
}

/// 判断该节点被赋值后不等于 0，避免除以零和 0^0 导致未定义的情况
fn is_not_zero(var: &'static str) -> impl Fn(&mut egg::EGraph<MathExpr, MathAnalysis>, egg::Id, &egg::Subst) -> bool {
    let var_sym: egg::Var = var.parse().unwrap();
    move |egraph, _, subst| {
        let id = subst[var_sym];
        if let Some(val) = egraph[id].data {
            val.into_inner() != 0.0
        } else {
            // 如果它不是常数，我们无法轻易断定它是否等于零，在保守起见下暂不进行折叠
            false
        }
    }
}

/// 判断节点是否为纯原子符号变量（而非数值常量）
/// CAS 系统通常假设符号变量在其定义域内为非零，因此可以安全地进行 (x*y)/x => y 简化
fn is_symbol_atom(var: &'static str) -> impl Fn(&mut egg::EGraph<MathExpr, MathAnalysis>, egg::Id, &egg::Subst) -> bool {
    let var_sym: egg::Var = var.parse().unwrap();
    move |egraph, _, subst| {
        let id = subst[var_sym];
        // 必须是纯 Symbol 节点，且没有常量值（排除数值 0 被意外匹配）
        egraph[id].nodes.iter().any(|n| matches!(n, MathExpr::Symbol(_)))
            && egraph[id].data.is_none()
    }
}

// ==================
// 5. 提取器与公共求导 API (API layer)
// ==================

/// 自定义一个成本函数，极度惩罚 `diff` 算子，强迫 Extractor 提取出已经求导完毕的表达式
pub struct DiffCost;
impl egg::CostFunction<MathExpr> for DiffCost {
    type Cost = usize;
    fn cost<C>(&mut self, enode: &MathExpr, mut costs: C) -> Self::Cost
    where
        C: FnMut(egg::Id) -> Self::Cost,
    {
        let mut cost = enode.fold(1, |acc, id| acc + costs(id));
        match enode {
            MathExpr::Diff(_) => cost += 1000, // 极大地惩罚含有求导算子的节点
            MathExpr::Add(_) | MathExpr::Sub(_) => cost += 2,     // 惩罚加减法，让系统偏好因式分解 (* 2 x) > (+ x x)
            MathExpr::Mul(_) | MathExpr::Div(_) => cost += 1,     // 稍微惩罚乘除法，让系统偏好指数 (pow x 2) > (* x x)
            _ => {}
        }
        cost
    }
}

/// 对表达式进行化简
pub fn simplify(expr: &RecExpr<MathExpr>) -> RecExpr<MathExpr> {
    let runner = Runner::default()
        .with_expr(expr)
        .run(&calculus_and_algebra_rules());

    let extractor = Extractor::new(&runner.egraph, DiffCost);
    let (_, best) = extractor.find_best(runner.roots[0]);
    best
}

/// 对给定的表达式进行求导，并自动化简结果
pub fn differentiate(expr: &RecExpr<MathExpr>, var: &str) -> RecExpr<MathExpr> {
    // 构造一个顶层的 `diff(expr, var)` 语法树
    let mut new_expr = expr.clone();

    // 获取当前表达式的根节点（最后一个加入的 Id）
    let expr_root = egg::Id::from(new_expr.as_ref().len() - 1);

    // 添加自变量符号节点
    let var_id = new_expr.add(MathExpr::Symbol(egg::Symbol::from(var)));

    // 添加 diff 节点
    new_expr.add(MathExpr::Diff([expr_root, var_id]));

    // 把带有 `diff` 节点的未计算表达式送进系统展开
    simplify(&new_expr)
}

/// 计算 n 阶导数（参考 Symbolics.jl Differential order 字段）
/// 通过递归调用 `differentiate` n 次实现
pub fn differentiate_n(expr: &RecExpr<MathExpr>, var: &str, n: usize) -> RecExpr<MathExpr> {
    let mut result = expr.clone();
    for _ in 0..n {
        result = differentiate(&result, var);
    }
    result
}

/// 计算表达式对多个变量的梯度向量 (参考 Symbolics.jl gradient 函数)
/// 返回 [∂f/∂x₁, ∂f/∂x₂, ..., ∂f/∂xₙ]
pub fn gradient(expr: &RecExpr<MathExpr>, vars: &[&str]) -> Vec<RecExpr<MathExpr>> {
    vars.iter().map(|v| differentiate(expr, v)).collect()
}

/// 计算表达式向量对变量向量的雅可比矩阵（参考 Symbolics.jl jacobian 函数）
/// 返回 J[i][j] = ∂exprs[i]/∂vars[j]
pub fn jacobian(exprs: &[RecExpr<MathExpr>], vars: &[&str]) -> Vec<Vec<RecExpr<MathExpr>>> {
    exprs.iter().map(|expr| gradient(expr, vars)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // 辅助函数，用来快速简化表达式并返回结果的字符串
    fn simplify_str(s: &str) -> String {
        let expr: RecExpr<MathExpr> = s.parse().unwrap();
        simplify(&expr).to_string()
    }

    fn diff_str(s: &str, var: &str) -> String {
        let expr: RecExpr<MathExpr> = s.parse().unwrap();
        differentiate(&expr, var).to_string()
    }

    #[test]
    fn test_trig_simplification() {
        // 测试毕达哥拉斯三角恒等式: sin^2(x) + cos^2(x) => 1
        assert_eq!(simplify_str("(+ (pow (sin x) 2) (pow (cos x) 2))"), "1");

        // 测试双曲恒等式: cosh^2(y) - sinh^2(y) => 1
        assert_eq!(simplify_str("(- (pow (cosh y) 2) (pow (sinh y) 2))"), "1");
    }

    #[test]
    fn test_constant_folding() {
        let s = "(sin (/ 0 2))";
        let expr: RecExpr<MathExpr> = s.parse().unwrap();
        let mut egraph = egg::EGraph::<MathExpr, MathAnalysis>::default();
        let id = egraph.add_expr(&expr);
        egraph.rebuild();

        assert_eq!(egraph[id].data.unwrap().into_inner(), 0.0);
    }

    #[test]
    fn test_constant_folding_new_fns() {
        // sec(0) = 1/cos(0) = 1
        let s = "(sec 0)";
        let expr: RecExpr<MathExpr> = s.parse().unwrap();
        let mut egraph = egg::EGraph::<MathExpr, MathAnalysis>::default();
        let id = egraph.add_expr(&expr);
        egraph.rebuild();
        assert_eq!(egraph[id].data.unwrap().into_inner(), 1.0);

        // log2(2) = 1
        let s2 = "(log2 2)";
        let expr2: RecExpr<MathExpr> = s2.parse().unwrap();
        let mut egraph2 = egg::EGraph::<MathExpr, MathAnalysis>::default();
        let id2 = egraph2.add_expr(&expr2);
        egraph2.rebuild();
        assert_eq!(egraph2[id2].data.unwrap().into_inner(), 1.0);

        // abs(-3) = 3
        let s3 = "(abs (neg 3))";
        let expr3: RecExpr<MathExpr> = s3.parse().unwrap();
        let mut egraph3 = egg::EGraph::<MathExpr, MathAnalysis>::default();
        let id3 = egraph3.add_expr(&expr3);
        egraph3.rebuild();
        assert_eq!(egraph3[id3].data.unwrap().into_inner(), 3.0);
    }

    #[test]
    fn test_algebraic_simplification() {
        // 合并同类项 x + x => 2 * x (由于乘法交换律，可能会变为 x * 2)
        let sum_x = simplify_str("(+ x x)");
        assert!(sum_x == "(* 2 x)" || sum_x == "(* x 2)");
        // x * x => x^2
        assert_eq!(simplify_str("(* x x)"), "(pow x 2)");
        // 指数与对数化简: exp(log(x)) => x
        assert_eq!(simplify_str("(exp (log x))"), "x");
        // 乘除法抵消（符号变量）: (x * y) / x => y
        let cancel = simplify_str("(/ (* x y) x)");
        assert_eq!(cancel, "y", "Expected '(* x y) / x' to simplify to 'y', got: {}", cancel);
        // 因式分解 (Factorization): x*y + x*z => x*(y+z)
        let factor = simplify_str("(+ (* x y) (* x z))");
        assert!(factor == "(* x (+ y z))" || factor == "(* (+ y z) x)");
    }

    #[test]
    fn test_derivative_basic() {
        // d/dx (neg x) = -1（修复前该规则被注释掉）
        let res = diff_str("(neg x)", "x");
        println!("d/dx(neg x) = {}", res);
        assert!(!res.contains("diff"), "d/dx(-x) should not contain 'diff', got: {}", res);

        // d/dx (y) = 0（不同变量偏导 = 0）
        let res2 = diff_str("y", "x");
        assert_eq!(res2, "0", "d/dx(y) should be 0, got: {}", res2);

        // d/dx (sin(x) * cos(x))
        let res3 = diff_str("(* (sin x) (cos x))", "x");
        println!("d/dx(sin(x)*cos(x)) = {}", res3);
        assert!(!res3.contains("diff"));

        // d/dx (pow x 2) = (* 2 x) 或等价
        let res4 = diff_str("(pow x 2)", "x");
        println!("d/dx(x^2) = {}", res4);
        assert!(!res4.contains("diff"));

        // d/dx (sqrt x)
        let res5 = diff_str("(sqrt x)", "x");
        println!("d/dx(sqrt(x)) = {}", res5);
        assert!(!res5.contains("diff"));
    }

    #[test]
    fn test_derivative_trig_new() {
        // d/dx sec(x) = sec(x)*tan(x) 或等价展开形式
        let res = diff_str("(sec x)", "x");
        println!("d/dx(sec(x)) = {}", res);
        assert!(!res.contains("diff"), "d/dx(sec(x)) should not contain 'diff', got: {}", res);

        // d/dx csc(x) = -csc(x)*cot(x) 或等价
        let res2 = diff_str("(csc x)", "x");
        println!("d/dx(csc(x)) = {}", res2);
        assert!(!res2.contains("diff"));

        // d/dx cot(x) = -csc(x)^2 或等价
        let res3 = diff_str("(cot x)", "x");
        println!("d/dx(cot(x)) = {}", res3);
        assert!(!res3.contains("diff"));

        // d/dx acot(x) = -1/(1+x^2) 或等价
        let res4 = diff_str("(acot x)", "x");
        println!("d/dx(acot(x)) = {}", res4);
        assert!(!res4.contains("diff"));
    }

    #[test]
    fn test_derivative_log2() {
        // d/dx log2(x) = 1/(x * ln(2))
        let res = diff_str("(log2 x)", "x");
        println!("d/dx(log2(x)) = {}", res);
        assert!(!res.contains("diff"), "d/dx(log2(x)) should not contain 'diff', got: {}", res);
    }

    #[test]
    fn test_derivative_pow_var() {
        // d/dx (x^x) — 通用幂法则 (u^v)' = u^v * (v'*ln(u) + v*u'/u)
        let res = diff_str("(pow x x)", "x");
        println!("d/dx(x^x) = {}", res);
        assert!(!res.contains("diff"), "d/dx(x^x) should not contain 'diff', got: {}", res);
    }

    #[test]
    fn test_higher_order_derivative() {
        // d²/dx² sin(x) = -sin(x) 或等价
        let expr: RecExpr<MathExpr> = "(sin x)".parse().unwrap();
        let d2 = differentiate_n(&expr, "x", 2);
        let d2_str = d2.to_string();
        println!("d²/dx²(sin(x)) = {}", d2_str);
        assert!(!d2_str.contains("diff"), "Second derivative of sin(x) should not contain 'diff', got: {}", d2_str);

        // d/dx x = 1
        let expr2: RecExpr<MathExpr> = "x".parse().unwrap();
        let d1 = differentiate_n(&expr2, "x", 1);
        assert_eq!(d1.to_string(), "1");

        // d³/dx³ x^3 = 6 (常数)
        let expr3: RecExpr<MathExpr> = "(pow x 3)".parse().unwrap();
        let d3 = differentiate_n(&expr3, "x", 3);
        let d3_str = d3.to_string();
        println!("d³/dx³(x^3) = {}", d3_str);
        assert!(!d3_str.contains("diff"), "Third derivative of x^3 should not contain 'diff', got: {}", d3_str);
    }

    #[test]
    fn test_gradient() {
        // grad(x^2 + y^2) = [2x, 2y]
        let expr: RecExpr<MathExpr> = "(+ (pow x 2) (pow y 2))".parse().unwrap();
        let grad = gradient(&expr, &["x", "y"]);
        assert_eq!(grad.len(), 2);
        let gx = grad[0].to_string();
        let gy = grad[1].to_string();
        println!("∂/∂x(x²+y²) = {}", gx);
        println!("∂/∂y(x²+y²) = {}", gy);
        assert!(!gx.contains("diff"));
        assert!(!gy.contains("diff"));
    }

    #[test]
    fn test_jacobian() {
        // J([x*y, x+y], [x, y]) should be 2x2 matrix without any 'diff'
        let f1: RecExpr<MathExpr> = "(* x y)".parse().unwrap();
        let f2: RecExpr<MathExpr> = "(+ x y)".parse().unwrap();
        let jac = jacobian(&[f1, f2], &["x", "y"]);
        assert_eq!(jac.len(), 2);
        assert_eq!(jac[0].len(), 2);
        for row in &jac {
            for entry in row {
                let s = entry.to_string();
                println!("J entry: {}", s);
                assert!(!s.contains("diff"));
            }
        }
    }

    #[test]
    fn test_derivative_original() {
        // 原有测试保留
        let res = simplify_str("(diff (* (sin x) (cos x)) x)");
        println!("Derivative result: {}", res);
        assert!(!res.contains("diff"), "Expected no 'diff' in the output, got: {}", res);

        let original: RecExpr<MathExpr> = "(* (sin x) (cos x))".parse().unwrap();
        let deriv = differentiate(&original, "x");
        let d_str = deriv.to_string();
        assert!(!d_str.contains("diff"));
    }
}

// ==============================================================
// Section 6: 变量替换 (Substitution)
// 对应 Symbolics.jl 的 substitute(expr, Dict(x => val, ...))
// ==============================================================

/// 将 `src` 的所有节点插入到 `dest`，返回新根节点的 Id。
/// 这是 substitute 的关键辅助函数，保证子树索引正确重新编号。
fn insert_rec_expr(dest: &mut RecExpr<MathExpr>, src: &RecExpr<MathExpr>) -> egg::Id {
    let mut id_map: Vec<egg::Id> = Vec::with_capacity(src.as_ref().len());
    for node in src.as_ref() {
        // map_children 将旧 Id 映射为新 Id
        let mapped = node.clone().map_children(|old_id| id_map[usize::from(old_id)]);
        id_map.push(dest.add(mapped));
    }
    *id_map.last().expect("src RecExpr must not be empty")
}

/// 将表达式中的符号变量替换为指定子表达式（批量）。
///
/// 对应 Symbolics.jl: `substitute(expr, Dict(x => a, y => b))`
///
/// # 示例
/// ```ignore
/// let expr: RecExpr<MathExpr> = "(+ x y)".parse().unwrap();
/// let a:    RecExpr<MathExpr> = "(sin z)".parse().unwrap();
/// let one:  RecExpr<MathExpr> = "1".parse().unwrap();
/// let result = substitute(&expr, &[("x", a), ("y", one)]);
/// // result ≈ "(+ (sin z) 1)"
/// ```ignore
pub fn substitute(
    expr: &RecExpr<MathExpr>,
    rules: &[(&str, RecExpr<MathExpr>)],
) -> RecExpr<MathExpr> {
    let mut new_expr: RecExpr<MathExpr> = Default::default();
    // id_map[old_index] = new Id（原节点在 new_expr 中对应的 Id）
    let mut id_map: Vec<egg::Id> = Vec::with_capacity(expr.as_ref().len());

    for node in expr.as_ref() {
        let new_id = if let MathExpr::Symbol(sym) = node {
            let sym_str = sym.as_str();
            if let Some((_, replacement)) = rules.iter().find(|(s, _)| *s == sym_str) {
                // 将整个替换子树插入，返回其根节点
                insert_rec_expr(&mut new_expr, replacement)
            } else {
                new_expr.add(MathExpr::Symbol(*sym))
            }
        } else {
            // 非符号节点：更新子节点引用后插入
            let mapped = node.clone().map_children(|old_id| id_map[usize::from(old_id)]);
            new_expr.add(mapped)
        };
        id_map.push(new_id);
    }
    // 最终做一轮化简（触发常量折叠等）
    simplify(&new_expr)
}

/// 将表达式中的符号变量替换为 f64 数值。
///
/// 对应 Symbolics.jl: `substitute(f, x => 2.0)`
///
/// # 示例
/// ```ignore
/// let expr: RecExpr<MathExpr> = "(+ (* 2 x) 1)".parse().unwrap();
/// let result = substitute_num(&expr, "x", 3.0);
/// // result = "7" (常量折叠后)
/// ```ignore
pub fn substitute_num(expr: &RecExpr<MathExpr>, var: &str, val: f64) -> RecExpr<MathExpr> {
    if let Ok(n) = ordered_float::NotNan::new(val) {
        let mut num_expr: RecExpr<MathExpr> = Default::default();
        num_expr.add(MathExpr::Num(n));
        substitute(expr, &[(var, num_expr)])
    } else {
        expr.clone() // NaN 不替换
    }
}

// ==============================================================
// Section 7: 代数展开 (Expand)
// 对应 Symbolics.jl 的 expand(expr)
// 注：使用直接递归树变换，避免 E-Graph 代价函数的"偏好压缩"问题
// ==============================================================

/// 递归地将乘法分配到加减法（分配律展开）。
/// 返回插入到 `out` 中的新根节点 Id。
fn expand_node(
    src_nodes: &[MathExpr],
    src_id: egg::Id,
    out: &mut RecExpr<MathExpr>,
) -> egg::Id {
    match &src_nodes[usize::from(src_id)] {
        MathExpr::Mul([a, b]) => {
            let a_id = expand_node(src_nodes, *a, out);
            let b_id = expand_node(src_nodes, *b, out);
            distribute_mul(a_id, b_id, out)
        }
        // 其他所有节点：递归子节点后插入
        node => {
            let mapped = node.clone().map_children(|ch| expand_node(src_nodes, ch, out));
            out.add(mapped)
        }
    }
}

/// 对已插入到 `out` 中的两个节点做分配律乘法。
/// 处理: Add, Sub, Neg 三种可以被分配的形式。
fn distribute_mul(a_id: egg::Id, b_id: egg::Id, out: &mut RecExpr<MathExpr>) -> egg::Id {
    // 克隆节点（避免借用冲突）
    let a_node = out.as_ref()[usize::from(a_id)].clone();
    let b_node = out.as_ref()[usize::from(b_id)].clone();

    match (a_node, b_node) {
        // (a1 + a2) * b  => a1*b + a2*b
        (MathExpr::Add([a1, a2]), _) => {
            let t1 = distribute_mul(a1, b_id, out);
            let t2 = distribute_mul(a2, b_id, out);
            out.add(MathExpr::Add([t1, t2]))
        }
        // a * (b1 + b2)  => a*b1 + a*b2
        (_, MathExpr::Add([b1, b2])) => {
            let t1 = distribute_mul(a_id, b1, out);
            let t2 = distribute_mul(a_id, b2, out);
            out.add(MathExpr::Add([t1, t2]))
        }
        // (a1 - a2) * b  => a1*b - a2*b
        (MathExpr::Sub([a1, a2]), _) => {
            let t1 = distribute_mul(a1, b_id, out);
            let t2 = distribute_mul(a2, b_id, out);
            out.add(MathExpr::Sub([t1, t2]))
        }
        // a * (b1 - b2)  => a*b1 - a*b2
        (_, MathExpr::Sub([b1, b2])) => {
            let t1 = distribute_mul(a_id, b1, out);
            let t2 = distribute_mul(a_id, b2, out);
            out.add(MathExpr::Sub([t1, t2]))
        }
        // (-a) * b  => -(a*b)
        (MathExpr::Neg(a1), _) => {
            let t = distribute_mul(a1, b_id, out);
            out.add(MathExpr::Neg(t))
        }
        // a * (-b)  => -(a*b)
        (_, MathExpr::Neg(b1)) => {
            let t = distribute_mul(a_id, b1, out);
            out.add(MathExpr::Neg(t))
        }
        // 无法继续分配：直接相乘
        _ => out.add(MathExpr::Mul([a_id, b_id])),
    }
}

/// 将乘法完全展开（分配律），类似 Symbolics.jl 的 `expand(expr)`。
///
/// 仅展开乘法对加减法的分配，不做幂次展开（如 (x+1)^2 不展开）。
///
/// # 示例
/// ```ignore
/// let e: RecExpr<MathExpr> = "(* x (+ y z))".parse().unwrap();
/// let r = expand(&e);
/// // r = "(+ (* x y) (* x z))"
/// ```ignore
pub fn expand(expr: &RecExpr<MathExpr>) -> RecExpr<MathExpr> {
    let src_nodes = expr.as_ref();
    let root_src = egg::Id::from(src_nodes.len() - 1);
    let mut out: RecExpr<MathExpr> = Default::default();
    let _ = expand_node(src_nodes, root_src, &mut out);
    simplify(&out) // 最后做常量折叠和化简
}

// ==============================================================
// Section 8: 多项式次数 (Degree)
// 对应 Symbolics.jl 的 degree(expr, var) in utils.jl
// ==============================================================

/// 计算表达式关于变量 `var_sym` 的多项式次数（递归实现）。
/// 返回 `i32::MAX` 表示超越函数依赖（不是多项式）。
fn compute_degree(nodes: &[MathExpr], id: egg::Id, var_sym: egg::Symbol) -> i32 {
    match &nodes[usize::from(id)] {
        MathExpr::Symbol(s)   => if *s == var_sym { 1 } else { 0 },
        MathExpr::Num(_)      => 0,

        MathExpr::Pow([base, exp]) => {
            let base_deg = compute_degree(nodes, *base, var_sym);
            if base_deg == 0 { return 0; }
            match &nodes[usize::from(*exp)] {
                MathExpr::Num(n) => {
                    let v = n.into_inner();
                    if v.fract() == 0.0 && v >= 0.0 {
                        (base_deg as f64 * v) as i32
                    } else {
                        i32::MAX // 分数/负数指数：非多项式
                    }
                }
                _ => i32::MAX, // 变量指数：非多项式
            }
        }
        MathExpr::Mul([a, b]) => {
            let da = compute_degree(nodes, *a, var_sym);
            let db = compute_degree(nodes, *b, var_sym);
            if da == i32::MAX || db == i32::MAX { i32::MAX } else { da + db }
        }
        MathExpr::Add([a, b]) | MathExpr::Sub([a, b]) => {
            compute_degree(nodes, *a, var_sym).max(compute_degree(nodes, *b, var_sym))
        }
        MathExpr::Neg(a) | MathExpr::Abs(a) => compute_degree(nodes, *a, var_sym),
        MathExpr::Div([num, den]) => {
            // 只有分母不含 var 时才考虑为有理式
            if compute_degree(nodes, *den, var_sym) == 0 {
                compute_degree(nodes, *num, var_sym)
            } else {
                i32::MAX
            }
        }
        // 超越函数：若参数含 var，则为超越依赖
        MathExpr::Sin(a) | MathExpr::Cos(a) | MathExpr::Tan(a) |
        MathExpr::Asin(a) | MathExpr::Acos(a) | MathExpr::Atan(a) |
        MathExpr::Sec(a) | MathExpr::Csc(a) | MathExpr::Cot(a) |
        MathExpr::Asec(a) | MathExpr::Acsc(a) | MathExpr::Acot(a) |
        MathExpr::Sinh(a) | MathExpr::Cosh(a) | MathExpr::Tanh(a) |
        MathExpr::Asinh(a) | MathExpr::Acosh(a) | MathExpr::Atanh(a) |
        MathExpr::Exp(a) | MathExpr::Log(a) | MathExpr::Log2(a) | MathExpr::Sqrt(a) |
        MathExpr::Sign(a) | MathExpr::Floor(a) | MathExpr::Ceil(a) => {
            if compute_degree(nodes, *a, var_sym) > 0 { i32::MAX } else { 0 }
        }
        MathExpr::Diff(_) => i32::MAX,
    }
}

/// 返回表达式对指定变量的多项式次数。
///
/// 返回 `i32::MAX` 表示超越函数依赖（`sin(x)` 等），`-1` 不会出现。
///
/// 对应 Symbolics.jl: `degree(expr, var)` in utils.jl
///
/// # 示例
/// ```ignore
/// let e: RecExpr<MathExpr> = "(+ (pow x 3) (* 2 x))".parse().unwrap();
/// assert_eq!(degree(&e, "x"), 3);
/// ```ignore
pub fn degree(expr: &RecExpr<MathExpr>, var: &str) -> i32 {
    let simplified = simplify(expr);
    let nodes = simplified.as_ref();
    let root = egg::Id::from(nodes.len() - 1);
    compute_degree(nodes, root, egg::Symbol::from(var))
}

/// 检查表达式对指定变量是否线性（次数 ≤ 1）。
/// 对应 Symbolics.jl: `islinear(expr, var)`
pub fn is_linear(expr: &RecExpr<MathExpr>, var: &str) -> bool {
    let d = degree(expr, var);
    d != i32::MAX && d <= 1
}

/// 检查表达式对所有给定变量是否仿射（各变量线性，不含交叉项检查）。
/// 注：对 `x*y`，分别对 x 和 y 的 `is_linear` 均返回 true，
/// 但 `x*y` 并非关于 `[x,y]` 的仿射函数（需 `hessian` 零矩阵才严格正确）。
/// 对应 Symbolics.jl: `isaffine(expr, vars)`
pub fn is_affine(expr: &RecExpr<MathExpr>, vars: &[&str]) -> bool {
    vars.iter().all(|v| is_linear(expr, v))
}

// ==============================================================
// Section 9: 数值求值 / 可调用函数 (build_function)
// 对应 Symbolics.jl 的 build_function(expr, vars...) 解释器版本
// ==============================================================

/// 递归求值单个节点（所有未知变量返回 NaN）。
fn eval_node(nodes: &[MathExpr], id: egg::Id, var_names: &[egg::Symbol], vals: &[f64]) -> f64 {
    let lookup = |sym: &egg::Symbol| -> f64 {
        var_names.iter().zip(vals.iter())
            .find(|(v, _)| *v == sym)
            .map(|(_, &n)| n)
            .unwrap_or(f64::NAN)
    };

    match &nodes[usize::from(id)] {
        MathExpr::Num(n)       => n.into_inner(),
        MathExpr::Symbol(sym)  => lookup(sym),

        MathExpr::Add([a, b])  => eval_node(nodes, *a, var_names, vals) + eval_node(nodes, *b, var_names, vals),
        MathExpr::Sub([a, b])  => eval_node(nodes, *a, var_names, vals) - eval_node(nodes, *b, var_names, vals),
        MathExpr::Mul([a, b])  => eval_node(nodes, *a, var_names, vals) * eval_node(nodes, *b, var_names, vals),
        MathExpr::Div([a, b])  => {
            let d = eval_node(nodes, *b, var_names, vals);
            if d != 0.0 { eval_node(nodes, *a, var_names, vals) / d } else { f64::NAN }
        }
        MathExpr::Pow([a, b])  => eval_node(nodes, *a, var_names, vals).powf(eval_node(nodes, *b, var_names, vals)),
        MathExpr::Neg(a)       => -eval_node(nodes, *a, var_names, vals),
        MathExpr::Abs(a)       => eval_node(nodes, *a, var_names, vals).abs(),
        MathExpr::Sqrt(a)      => eval_node(nodes, *a, var_names, vals).sqrt(),
        MathExpr::Sign(a)      => eval_node(nodes, *a, var_names, vals).signum(),
        MathExpr::Floor(a)     => eval_node(nodes, *a, var_names, vals).floor(),
        MathExpr::Ceil(a)      => eval_node(nodes, *a, var_names, vals).ceil(),

        MathExpr::Sin(a)       => eval_node(nodes, *a, var_names, vals).sin(),
        MathExpr::Cos(a)       => eval_node(nodes, *a, var_names, vals).cos(),
        MathExpr::Tan(a)       => eval_node(nodes, *a, var_names, vals).tan(),
        MathExpr::Asin(a)      => eval_node(nodes, *a, var_names, vals).asin(),
        MathExpr::Acos(a)      => eval_node(nodes, *a, var_names, vals).acos(),
        MathExpr::Atan(a)      => eval_node(nodes, *a, var_names, vals).atan(),
        MathExpr::Sec(a)       => 1.0 / eval_node(nodes, *a, var_names, vals).cos(),
        MathExpr::Csc(a)       => 1.0 / eval_node(nodes, *a, var_names, vals).sin(),
        MathExpr::Cot(a)       => {
            let v = eval_node(nodes, *a, var_names, vals);
            v.cos() / v.sin()
        }
        MathExpr::Asec(a)      => (1.0 / eval_node(nodes, *a, var_names, vals)).acos(),
        MathExpr::Acsc(a)      => (1.0 / eval_node(nodes, *a, var_names, vals)).asin(),
        MathExpr::Acot(a)      => (1.0 / eval_node(nodes, *a, var_names, vals)).atan(),

        MathExpr::Sinh(a)      => eval_node(nodes, *a, var_names, vals).sinh(),
        MathExpr::Cosh(a)      => eval_node(nodes, *a, var_names, vals).cosh(),
        MathExpr::Tanh(a)      => eval_node(nodes, *a, var_names, vals).tanh(),
        MathExpr::Asinh(a)     => eval_node(nodes, *a, var_names, vals).asinh(),
        MathExpr::Acosh(a)     => eval_node(nodes, *a, var_names, vals).acosh(),
        MathExpr::Atanh(a)     => eval_node(nodes, *a, var_names, vals).atanh(),

        MathExpr::Exp(a)       => eval_node(nodes, *a, var_names, vals).exp(),
        MathExpr::Log(a)       => eval_node(nodes, *a, var_names, vals).ln(),
        MathExpr::Log2(a)      => eval_node(nodes, *a, var_names, vals).log2(),

        MathExpr::Diff(_)      => f64::NAN, // 未求导的算子无法数值求值
    }
}

/// 将符号表达式编译为数值可调用函数（解释器版本）。
///
/// 对应 Symbolics.jl: `build_function(expr, vars...)` 的解释器形式（无 JIT）。
///
/// 返回的闭包接受 `&[f64]`（与 `vars` 顺序对应）并返回 `f64`。
///
/// # 示例
/// ```ignore
/// let expr: RecExpr<MathExpr> = "(+ (* 2 x) (pow x 2))".parse().unwrap();
/// let f = build_function(&expr, &["x"]);
/// assert!((f(&[3.0]) - 15.0).abs() < 1e-10); // 2*3 + 3^2 = 15
/// ```ignore
pub fn build_function(
    expr: &RecExpr<MathExpr>,
    vars: &[&str],
) -> impl Fn(&[f64]) -> f64 {
    let expr_clone = simplify(expr); // 预先化简
    let var_syms: Vec<egg::Symbol> = vars.iter().map(|&s| egg::Symbol::from(s)).collect();

    move |vals: &[f64]| {
        let nodes = expr_clone.as_ref();
        let root = egg::Id::from(nodes.len() - 1);
        eval_node(nodes, root, &var_syms, vals)
    }
}

// ==============================================================
// Section 10: Hessian 矩阵
// 对应 Symbolics.jl 的 hessian(expr, vars) in diff.jl
// ==============================================================

/// 计算 f 关于 vars 的 Hessian 矩阵（H[i][j] = ∂²f / ∂xᵢ∂xⱼ）。
///
/// 直接组合现有的 `gradient` 和 `jacobian` API 实现。
/// 对应 Symbolics.jl: `hessian(expr, vars)` in diff.jl
///
/// # 示例
/// ```ignore
/// let f: RecExpr<MathExpr> = "(+ (* x x) (* y y))".parse().unwrap();
/// let h = hessian(&f, &["x", "y"]);
/// // h = [[2, 0], [0, 2]]
/// ```ignore
pub fn hessian(expr: &RecExpr<MathExpr>, vars: &[&str]) -> Vec<Vec<RecExpr<MathExpr>>> {
    let grad = gradient(expr, vars); // ∇f = [∂f/∂x₀, ∂f/∂x₁, ...]
    jacobian(&grad, vars)            // J(∇f) = H
}

// ==============================================================
// Section 11: 泰勒展开 (Taylor Series)
// 对应 Symbolics.jl 的 taylor_coeff / taylor in taylor.jl
// ==============================================================

/// 计算 f 在给定变量 x=0 处的第 n 阶泰勒系数：cₙ = f⁽ⁿ⁾(0) / n!
///
/// 对应 Symbolics.jl: `taylor_coeff(f, x, n)` in taylor.jl
///
/// # 示例
/// ```ignore
/// let f: RecExpr<MathExpr> = "(exp x)".parse().unwrap();
/// // taylor_coeff(exp(x), x, 2) = exp(0)/2! = 1/2 = 0.5
/// let c2 = taylor_coeff(&f, "x", 2);
/// ```ignore
pub fn taylor_coeff(expr: &RecExpr<MathExpr>, var: &str, n: usize) -> RecExpr<MathExpr> {
    // 1. 求 n 阶导数
    let dn = differentiate_n(expr, var, n);
    // 2. 代入 x = 0
    let at_zero = substitute_num(&dn, var, 0.0);

    if n == 0 {
        return at_zero;
    }

    // 3. 除以 n! — 构造 (at_zero / n!) 节点
    let n_factorial = (1..=n).product::<usize>() as f64;
    // 尝试直接数值化简
    if let Some(MathExpr::Num(v)) = at_zero.as_ref().last() {
        let coeff = v.into_inner() / n_factorial;
        let mut result: RecExpr<MathExpr> = Default::default();
        result.add(MathExpr::Num(
            ordered_float::NotNan::new(coeff).unwrap_or_default(),
        ));
        return result;
    }

    // 无法数值化简时保留符号表示
    let mut result = at_zero;
    let num_id = egg::Id::from(result.as_ref().len() - 1);
    let nfact_nn = ordered_float::NotNan::new(n_factorial).expect("n! is always finite");
    let den_id = result.add(MathExpr::Num(nfact_nn));
    result.add(MathExpr::Div([num_id, den_id]));
    simplify(&result)
}

/// 构造 f 在 x=0 处展开到 n_max 阶的泰勒多项式。
///
/// 对应 Symbolics.jl: `taylor(f, x, 0:n_max)` in taylor.jl
///
/// # 示例
/// ```ignore
/// let f: RecExpr<MathExpr> = "(sin x)".parse().unwrap();
/// let t = taylor(&f, "x", 3);
/// // t ≈ x - x^3/6  (sin 的 3 阶泰勒展开)
/// ```ignore
pub fn taylor(expr: &RecExpr<MathExpr>, var: &str, n_max: usize) -> RecExpr<MathExpr> {
    let var_sym = egg::Symbol::from(var);

    let mut acc: Option<RecExpr<MathExpr>> = None;

    for n in 0..=n_max {
        let coeff = taylor_coeff(expr, var, n);

        // 检查系数是否为 0，若是则跳过
        if let Some(MathExpr::Num(v)) = coeff.as_ref().last() {
            if v.into_inner() == 0.0 {
                if acc.is_none() {
                    // 全为 0 时保留 0 项
                    acc = Some(coeff);
                }
                continue;
            }
        }

        // 构造 coeff * x^n 项
        let term = if n == 0 {
            coeff
        } else {
            let mut t = coeff;
            let coeff_id = egg::Id::from(t.as_ref().len() - 1);
            let var_id = t.add(MathExpr::Symbol(var_sym));
            let n_id = t.add(MathExpr::Num(
                ordered_float::NotNan::new(n as f64).unwrap(),
            ));
            let pow_id = t.add(MathExpr::Pow([var_id, n_id]));
            t.add(MathExpr::Mul([coeff_id, pow_id]));
            simplify(&t)
        };

        // 累计求和
        acc = Some(match acc {
            None => term,
            Some(prev) => {
                let mut combined = prev;
                let a_id = egg::Id::from(combined.as_ref().len() - 1);
                let b_id = insert_rec_expr(&mut combined, &term);
                combined.add(MathExpr::Add([a_id, b_id]));
                simplify(&combined)
            }
        });
    }

    // 如果全为 0 返回数字 0
    acc.unwrap_or_else(|| {
        let mut e: RecExpr<MathExpr> = Default::default();
        e.add(MathExpr::Num(ordered_float::NotNan::default()));
        e
    })
}

// ==============================================================
// Section 12: 系数提取 (Coefficient Extraction)
// 对应 Symbolics.jl 的 coeff(expr, var) in utils.jl
// ==============================================================

/// 提取多项式中 var 的一次项系数（即对 var 求导后在 var=0 处的值）。
///
/// 对应 Symbolics.jl: `coeff(expr, var)` （线性系数）
///
/// # 示例
/// ```ignore
/// let e: RecExpr<MathExpr> = "(+ (* 3 x) (* 2 y))".parse().unwrap();
/// // coeff_of(&e, "x") = 3
/// // coeff_of(&e, "y") = 2
/// ```ignore
pub fn coeff_of(expr: &RecExpr<MathExpr>, var: &str) -> RecExpr<MathExpr> {
    // 利用泰勒系数：c₁ = f'(0) / 1! = f'(0)
    taylor_coeff(expr, var, 1)
}

/// 提取多项式中 var^n 项的系数（即 n 阶泰勒系数）。
/// 对应 Symbolics.jl: `coeff(expr, var^n)`
pub fn coeff_of_n(expr: &RecExpr<MathExpr>, var: &str, n: usize) -> RecExpr<MathExpr> {
    taylor_coeff(expr, var, n)
}

// ==============================================================
// Section 13: 方程式 (Equation)
// 对应 Symbolics.jl 的 Equation 类型（equations.jl）
// ==============================================================

/// 符号等式 `lhs ~ rhs`，对应 Symbolics.jl 的 `Equation` struct。
///
/// # 示例
/// ```ignore
/// let lhs: RecExpr<MathExpr> = "(pow x 2)".parse().unwrap();
/// let rhs: RecExpr<MathExpr> = "1".parse().unwrap();
/// let eq = Equation::new(lhs, rhs); // x² ~ 1
/// ```ignore
#[derive(Clone, Debug)]
pub struct Equation {
    pub lhs: RecExpr<MathExpr>,
    pub rhs: RecExpr<MathExpr>,
}

impl Equation {
    /// 构造等式 `lhs ~ rhs`
    pub fn new(lhs: RecExpr<MathExpr>, rhs: RecExpr<MathExpr>) -> Self {
        Self { lhs, rhs }
    }

    /// 从字符串解析构造等式（便捷方法）
    pub fn parse(lhs_str: &str, rhs_str: &str) -> Self {
        Self {
            lhs: lhs_str.parse().expect("invalid lhs expression"),
            rhs: rhs_str.parse().expect("invalid rhs expression"),
        }
    }

    /// 化简等式两侧
    pub fn simplify_both(&self) -> Self {
        Self {
            lhs: simplify(&self.lhs),
            rhs: simplify(&self.rhs),
        }
    }

    /// 转换为规范形式 `lhs - rhs`（令其等于 0）
    /// 对应 Symbolics.jl: `canonical_form(eq)`
    pub fn to_canonical(&self) -> RecExpr<MathExpr> {
        let mut result = self.lhs.clone();
        let a_id = egg::Id::from(result.as_ref().len() - 1);
        let b_id = insert_rec_expr(&mut result, &self.rhs);
        result.add(MathExpr::Sub([a_id, b_id]));
        simplify(&result)
    }

    /// 数值求值：给定变量赋值后判断等式是否成立（误差 ≤ ε）
    /// 对应 Symbolics.jl: `evaluate(eq, Dict(x => val, ...))`
    pub fn evaluate(&self, subs: &[(&str, f64)]) -> bool {
        let vars: Vec<&str> = subs.iter().map(|(v, _)| *v).collect();
        let vals: Vec<f64>  = subs.iter().map(|(_, n)| *n).collect();
        let f_lhs = build_function(&self.lhs, &vars);
        let f_rhs = build_function(&self.rhs, &vars);
        (f_lhs(&vals) - f_rhs(&vals)).abs() < 1e-10
    }
}

impl std::fmt::Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ~ {}", self.lhs, self.rhs)
    }
}
