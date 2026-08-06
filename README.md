# Abstract Algebra Computational Engine

A strongly typed computational group theory engine written in Rust. This engine provides generic algebraic traits, concrete structures for common groups, and automated graph-search algorithms for generating group spaces and testing algebraic properties.

## Features

* **Generic Group Trait**: Defines core algebraic primitives (`combine`, `inverse`, `identity`) with flexible type bounds.
* **Automated Group Closure (`generate_group`)**: Generates complete finite groups from a minimal set of generators using an $O(N)$ `HashSet` Breadth-First Search (BFS) algorithm.
* **Subgroup & Normality Algorithms**:
  * `is_subgroup`: Validates algebraic closure over finite subsets.
  * `is_normal`: Automated verification of conjugate closure ($g n g^{-1} \in N, \forall g \in G, n \in N$).
* **Implemented Structures**:
  * `Symmetric<const N: usize>`: Permutation groups ($S_n$, $A_n$) using const generics.
  * `ZpMult<const P: usize>`: Multiplicative groups of integers modulo $p$.
  * `GL<const N: usize>`: General Linear Groups over complex numbers $GL_n(\mathbb{C})$.

## Trait Requirements

For hash-based group generation and dynamic set lookups, finite group elements generally implement:
`Group + Clone + Eq + Hash`

*(Note: Continuous or floating-point groups like $GL_n(\mathbb{C})$ omit `Eq` and `Hash` due to IEEE 754 floating-point constraints).*

## Usage Example

Generating $S_5$ (120 elements) from two minimal generators (a transposition and a 5-cycle) and checking subgroup normality:

```rust
use math_algebra_engine::structures::groups::Symmetric;
use math_algebra_engine::structures::helpers::{generate_group, is_normal};

fn main() {
    // Define generators for S5: transposition (0 1) and 5-cycle (0 1 2 3 4)
    let transposition = Symmetric::<5>::new(vec![1, 0, 2, 3, 4]).unwrap();
    let cycle = Symmetric::<5>::new(vec![1, 2, 3, 4, 0]).unwrap();

    // Generate full S5 group via BFS closure
    let s5: Vec<_> = generate_group::<Symmetric<5>>(&[transposition, cycle])
        .into_iter()
        .collect();

    assert_eq!(s5.len(), 120);

    // Verify A5 is normal in S5
    // assert!(is_normal::<Symmetric<5>>(&s5, &a5));
}
