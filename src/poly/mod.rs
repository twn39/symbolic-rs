use num_rational::BigRational;
use std::collections::HashMap;

/// 3. 多项式处理的核心：多变量多项式环 (Multivariate Polynomial Rings)
///
/// 当表达式被要求完全展开 (Expand) 时，树状结构性能灾难。
/// 我们用字典（Hash 表）将表达式转化为：系数 -> 指数集 的稀疏表示。
///
/// 这里表示 c * x^a * y^b * z^c ...
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Monomial {
    // 例如，对变量集合 (x, y, z)，其各自的指数序列：
    // 若这是 x^2 * y^1，则为 [2, 1, 0]
    pub exponents: Vec<u32>,
}

/// 基于散列表的抽象多项式表示 (Sparse Representation)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial {
    // 键是每个单项式的指数组合，值是多精度有理数系数
    pub terms: HashMap<Monomial, BigRational>,
}

impl Polynomial {
    pub fn new() -> Self {
        Self {
            terms: HashMap::new(),
        }
    }

    /// 两个多项式相加变成了 HashMap 的键值合并，这是 O(N) 的极速操作。
    pub fn add(mut self, other: &Self) -> Self {
        for (monomial, coeff) in &other.terms {
            let entry = self.terms.entry(monomial.clone()).or_insert_with(|| BigRational::from_integer(0.into()));
            *entry += coeff;
        }
        // TODO: 可在此处清理系数为 0 的项
        self
    }

    /// 若配合 Rayon，多项式乘法可以做成并行笛卡尔积。
    pub fn multiply(&self, _other: &Self) -> Self {
        // ... 代数乘法，指数相加，系数相乘 ...
        Self::new()
    }
}
