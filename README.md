# symbolic-rs

A high-performance symbolic algebra system and computer algebra system (CAS) written in Rust.

Instead of relying on traditional recursive Abstract Syntax Trees (ASTs), which suffer from exponential memory explosion and slow equivalence checks, this project utilizes **E-Graphs (Equivalence Graphs)** via the [`egg`](https://egraphs-good.github.io/) crate. 

By treating algebraic simplification and symbolic differentiation as a database-like search and rewrite problem, the system achieves extreme performance, O(1) equivalence checks, and memory deduplication (Hash Consing).

## Core Capabilities

* **Core Representation**: Defines the `MathExpr` language (Algebra, Trigonometry, Hyperbolic, Calculus) and implements automatic numerical constant folding.
* **Rewrite Engine**: A declarative rule engine containing mathematical properties (factorization, logarithm rules, trigonometric identities, differentiation). Exposes `simplify` and `differentiate` APIs.
* **LaTeX Generation**: Converts the simplified E-Graph ASTs into perfectly formatted LaTeX strings, featuring an intelligent precedence system.
* **Sparse Polynomials (WIP)**: Fast multivariate polynomial ring manipulation.
* **Pratt Parsing (WIP)**: Expression parsing using `winnow`.

## Building and Testing

The project is a standard Rust library managed by Cargo.

**Build the library:**
```bash
cargo build
```

**Run the test suite:**
```bash
cargo test
```
*Note: The test suite includes comprehensive tests for constant folding, algebraic simplification, derivatives, and LaTeX rendering.*

## Architecture & Internals

* `src/expr/mod.rs`: `MathExpr` representation and constant folding.
* `src/rules/mod.rs`: Rewrite rules and the `Extractor`'s Cost Functions (`DiffCost`).
* `src/latex/mod.rs`: Precedence-aware LaTeX string generation.
