#[cfg(test)]
mod test_trace {
    use symbolic_rs::expr::MathExpr;
    use symbolic_rs::expr::MathAnalysis;
    use symbolic_rs::rules::{calculus_and_algebra_rules, DiffCost};
    use egg::{Runner, Extractor, RecExpr};

    #[test]
    fn run() {
        let s = "(diff (* y x) x)";
        let expr: RecExpr<MathExpr> = s.parse().unwrap();
        let mut runner = Runner::<MathExpr, MathAnalysis, ()>::default()
            .with_expr(&expr)
            .run(&calculus_and_algebra_rules());
            
        let mut extractor = Extractor::new(&runner.egraph, DiffCost);
        let (_, best) = extractor.find_best(runner.roots[0]);
        println!("Best: {}", best);

        for c in runner.egraph.classes() {
            if let Some(d) = c.data {
                if d.into_inner().is_infinite() {
                    println!("INF FOUND IN CLASS {}", c.id);
                    for n in &c.nodes {
                        println!("  Node: {:?}", n);
                    }
                }
            }
        }
    }
}
