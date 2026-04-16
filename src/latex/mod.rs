use crate::expr::MathExpr;
use egg::RecExpr;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Prec {
    AddSub = 0,
    MulDiv = 1,
    Neg    = 2,
    Pow    = 3,
    Atom   = 4,
}

/// 遍历 E-Graph 提取出的最优 AST 并将其格式化为标准的 LaTeX 字符串。
/// 我们实现了一个简易的运算符优先级 (Precedence) 系统，使得括号的添加恰到好处。
pub fn to_latex(expr: &RecExpr<MathExpr>) -> String {
    // 按拓扑顺序逐一遍历解析好的 AST
    let mut results: Vec<(String, Prec)> = Vec::with_capacity(expr.as_ref().len());

    let wrap = |s: &str, prec: Prec, required_prec: Prec| -> String {
        if prec < required_prec {
            format!(r"\left({}\right)", s)
        } else {
            s.to_string()
        }
    };

    for node in expr.as_ref() {
        let (s, p) = match node {
            MathExpr::Num(n)    => (format!("{}", n), Prec::Atom),
            MathExpr::Symbol(sym) => (format!("{}", sym), Prec::Atom),

            MathExpr::Add([a, b]) => {
                let (sa, pa) = &results[usize::from(*a)];
                let (sb, pb) = &results[usize::from(*b)];
                (format!("{} + {}", wrap(sa, *pa, Prec::AddSub), wrap(sb, *pb, Prec::AddSub)), Prec::AddSub)
            }
            MathExpr::Sub([a, b]) => {
                let (sa, pa) = &results[usize::from(*a)];
                let (sb, pb) = &results[usize::from(*b)];
                let left  = wrap(sa, *pa, Prec::AddSub);
                // 对于减法右侧，如果是同级（如加法），也必须加上括号以防结合律错误 (a - (b + c))
                let right = if *pb <= Prec::AddSub { format!(r"\left({}\right)", sb) } else { sb.clone() };
                (format!("{} - {}", left, right), Prec::AddSub)
            }
            MathExpr::Mul([a, b]) => {
                let (sa, pa) = &results[usize::from(*a)];
                let (sb, pb) = &results[usize::from(*b)];
                (format!(r"{} \cdot {}", wrap(sa, *pa, Prec::MulDiv), wrap(sb, *pb, Prec::MulDiv)), Prec::MulDiv)
            }
            MathExpr::Div([a, b]) => {
                let (sa, _) = &results[usize::from(*a)];
                let (sb, _) = &results[usize::from(*b)];
                (format!(r"\frac{{{}}}{{{}}}", sa, sb), Prec::Atom) // \frac 自带块结构，无需外层括号
            }
            MathExpr::Pow([a, b]) => {
                let (sa, pa) = &results[usize::from(*a)];
                let (sb, _)  = &results[usize::from(*b)];
                let left = if *pa < Prec::Pow { format!(r"\left({}\right)", sa) } else { sa.clone() };
                (format!("{{{}}}^{{{}}}", left, sb), Prec::Pow)
            }
            MathExpr::Sqrt(a) => {
                let (sa, _) = &results[usize::from(*a)];
                (format!(r"\sqrt{{{}}}", sa), Prec::Atom)
            }
            MathExpr::Neg(a) => {
                let (sa, pa) = &results[usize::from(*a)];
                let inner = if *pa <= Prec::AddSub { format!(r"\left({}\right)", sa) } else { sa.clone() };
                (format!("-{}", inner), Prec::Neg)
            }
            MathExpr::Abs(a) => {
                let (sa, _) = &results[usize::from(*a)];
                (format!(r"\left|{}\right|", sa), Prec::Atom)
            }
            MathExpr::Sign(a) => {
                let (sa, _) = &results[usize::from(*a)];
                (format!(r"\operatorname{{sgn}}\left({}\right)", sa), Prec::Atom)
            }
            MathExpr::Floor(a) => {
                let (sa, _) = &results[usize::from(*a)];
                (format!(r"\left\lfloor {}\right\rfloor", sa), Prec::Atom)
            }
            MathExpr::Ceil(a) => {
                let (sa, _) = &results[usize::from(*a)];
                (format!(r"\left\lceil {}\right\rceil", sa), Prec::Atom)
            }

            // 基础三角函数
            MathExpr::Sin(a)  => (format!(r"\sin\left({}\right)",    results[usize::from(*a)].0), Prec::Atom),
            MathExpr::Cos(a)  => (format!(r"\cos\left({}\right)",    results[usize::from(*a)].0), Prec::Atom),
            MathExpr::Tan(a)  => (format!(r"\tan\left({}\right)",    results[usize::from(*a)].0), Prec::Atom),
            MathExpr::Asin(a) => (format!(r"\arcsin\left({}\right)", results[usize::from(*a)].0), Prec::Atom),
            MathExpr::Acos(a) => (format!(r"\arccos\left({}\right)", results[usize::from(*a)].0), Prec::Atom),
            MathExpr::Atan(a) => (format!(r"\arctan\left({}\right)", results[usize::from(*a)].0), Prec::Atom),
            // 正割/余割/余切
            MathExpr::Sec(a)  => (format!(r"\sec\left({}\right)",    results[usize::from(*a)].0), Prec::Atom),
            MathExpr::Csc(a)  => (format!(r"\csc\left({}\right)",    results[usize::from(*a)].0), Prec::Atom),
            MathExpr::Cot(a)  => (format!(r"\cot\left({}\right)",    results[usize::from(*a)].0), Prec::Atom),
            MathExpr::Asec(a) => (format!(r"\operatorname{{arcsec}}\left({}\right)", results[usize::from(*a)].0), Prec::Atom),
            MathExpr::Acsc(a) => (format!(r"\operatorname{{arccsc}}\left({}\right)", results[usize::from(*a)].0), Prec::Atom),
            MathExpr::Acot(a) => (format!(r"\operatorname{{arccot}}\left({}\right)", results[usize::from(*a)].0), Prec::Atom),

            // 双曲函数
            MathExpr::Sinh(a)  => (format!(r"\sinh\left({}\right)",                  results[usize::from(*a)].0), Prec::Atom),
            MathExpr::Cosh(a)  => (format!(r"\cosh\left({}\right)",                  results[usize::from(*a)].0), Prec::Atom),
            MathExpr::Tanh(a)  => (format!(r"\tanh\left({}\right)",                  results[usize::from(*a)].0), Prec::Atom),
            MathExpr::Asinh(a) => (format!(r"\operatorname{{arsinh}}\left({}\right)", results[usize::from(*a)].0), Prec::Atom),
            MathExpr::Acosh(a) => (format!(r"\operatorname{{arcosh}}\left({}\right)", results[usize::from(*a)].0), Prec::Atom),
            MathExpr::Atanh(a) => (format!(r"\operatorname{{artanh}}\left({}\right)", results[usize::from(*a)].0), Prec::Atom),

            // 指数与对数
            MathExpr::Exp(a)  => (format!(r"e^{{{}}}", results[usize::from(*a)].0), Prec::Pow),
            MathExpr::Log(a)  => (format!(r"\ln\left({}\right)",        results[usize::from(*a)].0), Prec::Atom),
            MathExpr::Log2(a) => (format!(r"\log_{{2}}\left({}\right)", results[usize::from(*a)].0), Prec::Atom),

            // 微积分求导算子
            MathExpr::Diff([expr_id, var_id]) => {
                let (s_expr, _) = &results[usize::from(*expr_id)];
                let (s_var, _)  = &results[usize::from(*var_id)];
                (format!(r"\frac{{d}}{{d {}}} \left[ {} \right]", s_var, s_expr), Prec::Atom)
            }
        };
        results.push((s, p));
    }

    results.last().map(|(s, _)| s.clone()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latex_generation() {
        // e^x + sin(x) / (x - 2)
        let s = "(+ (exp x) (/ (sin x) (- x 2)))";
        let expr: RecExpr<MathExpr> = s.parse().unwrap();
        let tex = to_latex(&expr);
        assert_eq!(tex, r"e^{x} + \frac{\sin\left(x\right)}{x - 2}");

        // d/dx [ sin(x) * cos(x) ]
        let s2 = "(diff (* (sin x) (cos x)) x)";
        let expr2: RecExpr<MathExpr> = s2.parse().unwrap();
        let tex2 = to_latex(&expr2);
        assert_eq!(tex2, r"\frac{d}{d x} \left[ \sin\left(x\right) \cdot \cos\left(x\right) \right]");

        // 测试括号优先级: (a + b) * c
        let s3 = "(* (+ a b) c)";
        let expr3: RecExpr<MathExpr> = s3.parse().unwrap();
        let tex3 = to_latex(&expr3);
        assert_eq!(tex3, r"\left(a + b\right) \cdot c");
    }

    #[test]
    fn test_latex_new_functions() {
        // sec(x)
        let s = "(sec x)";
        let expr: RecExpr<MathExpr> = s.parse().unwrap();
        assert_eq!(to_latex(&expr), r"\sec\left(x\right)");

        // csc(x)
        let s2 = "(csc x)";
        let expr2: RecExpr<MathExpr> = s2.parse().unwrap();
        assert_eq!(to_latex(&expr2), r"\csc\left(x\right)");

        // cot(x)
        let s3 = "(cot x)";
        let expr3: RecExpr<MathExpr> = s3.parse().unwrap();
        assert_eq!(to_latex(&expr3), r"\cot\left(x\right)");

        // log2(x)
        let s4 = "(log2 x)";
        let expr4: RecExpr<MathExpr> = s4.parse().unwrap();
        assert_eq!(to_latex(&expr4), r"\log_{2}\left(x\right)");

        // abs(x)
        let s5 = "(abs x)";
        let expr5: RecExpr<MathExpr> = s5.parse().unwrap();
        assert_eq!(to_latex(&expr5), r"\left|x\right|");

        // acot(x)
        let s6 = "(acot x)";
        let expr6: RecExpr<MathExpr> = s6.parse().unwrap();
        assert_eq!(to_latex(&expr6), r"\operatorname{arccot}\left(x\right)");
    }
}
