# Graph Coloring

**A Rust library implementing three graph coloring algorithms** — greedy sequential, DSatur (degree-of-saturation), and exact backtracking with pruning — to assign the minimum number of colors such that no adjacent vertices share a color.

## Why It Matters

Graph coloring is NP-complete and appears across computer science: register allocation in compilers (Chaitin's algorithm), exam scheduling, frequency assignment in cellular networks, map coloring (four-color theorem), and Sudoku solving. The minimum number of colors (the chromatic number χ(G)) is a fundamental graph invariant. In practice, exact coloring is exponential, so heuristic algorithms (greedy, DSatur) provide fast approximations, while backtracking finds the true optimum for small graphs.

## How It Works

**Greedy coloring** — **O(V + E)**: Process vertices in arbitrary order. For each vertex, scan its neighbors' assigned colors and pick the smallest unclaimed color. Simple and fast, but quality depends heavily on vertex ordering.

**DSatur coloring**: A smarter heuristic that processes vertices by **saturation degree** — the number of distinct colors among its neighbors. At each step, pick the uncolored vertex with highest saturation (breaking ties by degree), then assign the smallest available color. This naturally prioritizes constrained vertices first, typically using fewer colors than plain greedy.

**Exact coloring** — **O(k^V)** worst case: Tries 1, 2, 3, ... colors and uses backtracking with constraint propagation. For `k` colors, it assigns each vertex a color, checking that no neighbor has the same color. If a valid assignment is found, the chromatic number is `k`. Pruning: if any partial assignment leads to a conflict, backtrack immediately.

The graph uses a `HashMap<usize, HashSet<usize>>` adjacency representation.

## Quick Start

```rust
use graph_coloring::{Graph, greedy_coloring, dsatur_coloring, exact_coloring, chromatic_number};

fn main() {
    let mut g = Graph::new();
    g.add_edge(0, 1);
    g.add_edge(1, 2);
    g.add_edge(0, 2); // Triangle — needs 3 colors

    // Greedy: fast approximation
    let c = greedy_coloring(&g);
    println!("Greedy: {} colors", chromatic_number(&c));

    // DSatur: better heuristic
    let c = dsatur_coloring(&g);
    println!("DSatur: {} colors", chromatic_number(&c));

    // Exact: minimum chromatic number
    let c = exact_coloring(&g, 3).unwrap();
    println!("Exact: {} colors", chromatic_number(&c)); // 3 for a triangle
}
```

## API

| Function | Complexity | Description |
|---|---|---|
| `Graph::new()` | **O(1)** | Empty adjacency-list graph |
| `greedy_coloring(graph)` | **O(V + E)** | Smallest-available-color heuristic |
| `dsatur_coloring(graph)` | **O(V²)** | Saturation-degree heuristic |
| `exact_coloring(graph, max)` | **O(k^V)** | Exact chromatic number via backtracking |
| `chromatic_number(coloring)` | **O(n)** | Count distinct colors used |

## Architecture Notes

Part of the SuperInstance graph algorithms collection. Companion crates: `graph-bfs`, `graph-dfs`, `graph-dijkstra`, `graph-astar`, `graph-bellman-ford`. See the [Architecture Guide](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## License

MIT
