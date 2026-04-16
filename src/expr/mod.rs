use egg::{define_language, Id, Symbol, Analysis, EGraph};
use ordered_float::NotNan;

// 保证代数系统中的数字可以被 `egg` 安全地放入图里 (它需要 Hash, Eq, Ord)
pub type NumVal = NotNan<f64>;

// 1. Core Representation: 包含初等代数与三角双曲函数的语法定义
define_language! {
    pub enum MathExpr {
        // 基本数字与变量符号
        Num(NumVal),
        Symbol(Symbol),

        // 基础算术运算
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "*" = Mul([Id; 2]),
        "/" = Div([Id; 2]),
        "pow" = Pow([Id; 2]),
        "sqrt" = Sqrt(Id),
        "neg" = Neg(Id), // 一元负号: -x
        "abs"   = Abs(Id),   // 绝对值: |x|
        "sign"  = Sign(Id),  // 符号函数: sgn(x)
        "floor" = Floor(Id), // 向下取整: ⌊x⌋
        "ceil"  = Ceil(Id),  // 向上取整: ⌈x⌉

        // 三角函数 (Trigonometric)
        "sin"  = Sin(Id),
        "cos"  = Cos(Id),
        "tan"  = Tan(Id),
        "asin" = Asin(Id),
        "acos" = Acos(Id),
        "atan" = Atan(Id),
        // 正割/余割/余切 (参考 Symbolics.jl register_derivatives.jl)
        "sec"  = Sec(Id),
        "csc"  = Csc(Id),
        "cot"  = Cot(Id),
        "asec" = Asec(Id),
        "acsc" = Acsc(Id),
        "acot" = Acot(Id),

        // 双曲函数 (Hyperbolic)
        "sinh"  = Sinh(Id),
        "cosh"  = Cosh(Id),
        "tanh"  = Tanh(Id),
        "asinh" = Asinh(Id),
        "acosh" = Acosh(Id),
        "atanh" = Atanh(Id),

        // 指数与对数 (自然对数 + 以 2 为底)
        "exp"  = Exp(Id),
        "log"  = Log(Id),
        "log2" = Log2(Id),

        // 微积分算子：求导
        "diff" = Diff([Id; 2]),
    }
}

// ==========================================
// 2. 常量折叠引擎 (Numerical Constant Folding)
// ==========================================
// 引擎在运行代数推导时，只要任意子树完全是由常数构成的，
// 该分析器就会直接计算出其精确浮点数数值，避免无意义的代数展开。
#[derive(Default)]
pub struct MathAnalysis;

impl Analysis<MathExpr> for MathAnalysis {
    type Data = Option<NumVal>;

    fn merge(&mut self, a: &mut Self::Data, b: Self::Data) -> egg::DidMerge {
        // 合并节点时，我们保留常量信息 (如果某一分支能计算出常量)
        egg::merge_max(a, b)
    }

    fn make(egraph: &EGraph<MathExpr, Self>, enode: &MathExpr) -> Self::Data {
        // 辅助函数，快速获取子节点运算结果
        let x = |i: &Id| egraph[*i].data.map(|n| n.into_inner());

        match enode {
            MathExpr::Num(n)    => Some(*n),
            MathExpr::Symbol(_) => None,

            // 四则运算与代数常量折叠
            MathExpr::Add([a, b]) => NotNan::new(x(a)? + x(b)?).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Sub([a, b]) => NotNan::new(x(a)? - x(b)?).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Mul([a, b]) => NotNan::new(x(a)? * x(b)?).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Div([a, b]) => {
                let den = x(b)?;
                if den != 0.0 { NotNan::new(x(a)? / den).ok().filter(|n| n.into_inner().is_finite()) } else { None }
            }
            MathExpr::Pow([a, b]) => {
                let base = x(a)?;
                let exp  = x(b)?;
                if base == 0.0 && exp == 0.0 {
                    None
                } else {
                    NotNan::new(base.powf(exp)).ok().filter(|n| n.into_inner().is_finite())
                }
            }
            MathExpr::Sqrt(a) => NotNan::new(x(a)?.sqrt()).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Neg(a)  => NotNan::new(-x(a)?).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Abs(a)   => NotNan::new(x(a)?.abs()).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Sign(a)  => NotNan::new(x(a)?.signum()).ok(),
            MathExpr::Floor(a) => NotNan::new(x(a)?.floor()).ok(),
            MathExpr::Ceil(a)  => NotNan::new(x(a)?.ceil()).ok(),

            // 三角函数常量折叠
            MathExpr::Sin(a)  => NotNan::new(x(a)?.sin()).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Cos(a)  => NotNan::new(x(a)?.cos()).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Tan(a)  => NotNan::new(x(a)?.tan()).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Asin(a) => NotNan::new(x(a)?.asin()).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Acos(a) => NotNan::new(x(a)?.acos()).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Atan(a) => NotNan::new(x(a)?.atan()).ok().filter(|n| n.into_inner().is_finite()),
            // sec = 1/cos, csc = 1/sin, cot = cos/sin
            MathExpr::Sec(a) => {
                let c = x(a)?.cos();
                if c != 0.0 { NotNan::new(1.0 / c).ok().filter(|n| n.into_inner().is_finite()) } else { None }
            }
            MathExpr::Csc(a) => {
                let s = x(a)?.sin();
                if s != 0.0 { NotNan::new(1.0 / s).ok().filter(|n| n.into_inner().is_finite()) } else { None }
            }
            MathExpr::Cot(a) => {
                let s = x(a)?.sin();
                if s != 0.0 { NotNan::new(x(a)?.cos() / s).ok().filter(|n| n.into_inner().is_finite()) } else { None }
            }
            // 反三角: asec(x) = acos(1/x), acsc(x) = asin(1/x), acot(x) = atan(1/x)
            MathExpr::Asec(a) => {
                let v = x(a)?;
                if v != 0.0 { NotNan::new((1.0 / v).acos()).ok().filter(|n| n.into_inner().is_finite()) } else { None }
            }
            MathExpr::Acsc(a) => {
                let v = x(a)?;
                if v != 0.0 { NotNan::new((1.0 / v).asin()).ok().filter(|n| n.into_inner().is_finite()) } else { None }
            }
            MathExpr::Acot(a) => {
                NotNan::new(x(a)?.recip().atan()).ok().filter(|n| n.into_inner().is_finite())
            }

            // 双曲函数常量折叠
            MathExpr::Sinh(a)  => NotNan::new(x(a)?.sinh()).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Cosh(a)  => NotNan::new(x(a)?.cosh()).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Tanh(a)  => NotNan::new(x(a)?.tanh()).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Asinh(a) => NotNan::new(x(a)?.asinh()).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Acosh(a) => NotNan::new(x(a)?.acosh()).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Atanh(a) => NotNan::new(x(a)?.atanh()).ok().filter(|n| n.into_inner().is_finite()),

            // 指数与对数常量折叠
            MathExpr::Exp(a)  => NotNan::new(x(a)?.exp()).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Log(a)  => NotNan::new(x(a)?.ln()).ok().filter(|n| n.into_inner().is_finite()),
            MathExpr::Log2(a) => NotNan::new(x(a)?.log2()).ok().filter(|n| n.into_inner().is_finite()),

            // 包含微积分算符则不再被视为可以直接计算数值的常量
            MathExpr::Diff(_) => None,
            // Sign/Floor/Ceil 的常量折叠已在上方处理，
            // 此处仅作为编译完整性的保障（永不到达）
        }
    }

    fn modify(egraph: &mut EGraph<MathExpr, Self>, id: Id) {
        // 如果某一节点或子树成功计算出了浮点数常量
        // 并且不是无穷大或者 NaN
        if let Some(c) = egraph[id].data {
            if c.into_inner().is_finite() {
                let added = egraph.add(MathExpr::Num(c));
                egraph.union(id, added);
            }
        }
    }
}
