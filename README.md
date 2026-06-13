# graph-coloring: Greedy, DSatur, and Exact Graph Coloring

Three graph coloring algorithms with increasing accuracy and complexity: **greedy sequential** (O(V+E)), **DSatur** (saturation-degree heuristic), and **exact backtracking** (finds the chromatic number). Operates on adjacency-list graphs with `HashSet` neighbors for O(1) edge queries.

## Why It Matters

Graph coloring is one of the most fundamental problems in computer science, with direct applications in:

- **Register allocation**: Color interference graphs to map variables to CPU registers (Chaitin, 1981)
- **Scheduling**: Assign time slots to exams/meetings so no conflicts share a slot
- **Frequency assignment**: Color cell towers so adjacent towers use different frequencies
- **Map coloring**: The four-color theorem states χ(planar graph) ≤ 4 (Appel & Haken, 1976)
- **Sudoku**: Each puzzle is a graph coloring problem on 81 vertices with 9 colors

Finding the chromatic number (minimum colors) is **NP-complete** (Karp, 1972), so we need heuristics for practical instances and exponential exact algorithms for small ones.

## How It Works

### Greedy Sequential Coloring — O(V + E)

Process vertices in order, assigning the smallest color not used by any colored neighbor:

```
for each vertex v (in order):
    used = { color[u] : u ∈ N(v), u already colored }
    color[v] = min{ k ∈ ℕ : k ∉ used }
```

The number of colors used depends heavily on vertex ordering. For the complete graph K_n, any ordering uses n colors. For a path graph P_n, it uses exactly 2.

### DSatur (Degree of Saturation) — O(V²)

At each step, select the uncolored vertex with the highest **saturation degree** — the number of distinct colors among its neighbors. Break ties by ordinary degree.

```
while uncolored vertices remain:
    v = argmax { |{color[u] : u ∈ N(v)}| }  (tie: max degree)
    color[v] = smallest available color
    update saturation degrees of v's neighbors
```

DSatur uses at most Δ+1 colors (Δ = max degree) and frequently beats greedy on irregular graphs. Brélaz (1979) showed it produces optimal or near-optimal colorings on standard benchmarks.

### Exact Coloring (Backtracking) — O(k^V)

Try to color with k = 1, 2, 3, ... colors until success:

```
for k in 1..=max_colors:
    if backtrack(nodes, 0, k): return coloring
    
backtrack(idx, k):
    if idx == n: return true
    for color in 0..k:
        if no neighbor of nodes[idx] has this color:
            assign color
            if backtrack(idx+1, k): return true
            unassign
    return false
```

Pruning: the `max_colors` bound (typically from greedy + 1) avoids trying hopeless values of k.

### Chromatic Number

```
χ(G) = max(coloring.values()) + 1
```

### Complexity

| Algorithm | Time | Space | Guarantees |
|-----------|------|-------|------------|
| Greedy | O(V + E) | O(V) | ≤ Δ+1 colors |
| DSatur | O(V²) | O(V²) | ≤ Δ+1 colors, better in practice |
| Exact | O(k^V) | O(V) | Optimal χ(G) |

Where Δ = max degree, V = vertices, E = edges, k = chromatic number.

### Tested Examples

- **Triangle** (K₃): χ = 3 (all algorithms agree)
- **Complete bipartite** (K₂,₂): χ = 2 (DSatur and exact find optimal; greedy may use 2 or 3)

## Quick Start

```rust
use graph_coloring::{Graph, greedy_coloring, dsatur_coloring, exact_coloring, chromatic_number};

let mut g = Graph::new();
g.add_edge(0, 1);
g.add_edge(1, 2);
g.add_edge(0, 2); // Triangle K₃

let c = greedy_coloring(&g);
assert_eq!(chromatic_number(&c), 3);

let c_exact = exact_coloring(&g, 3).unwrap();
assert_eq!(chromatic_number(&c_exact), 3);
```

## API

| Function | Signature | Description |
|----------|-----------|-------------|
| `greedy_coloring` | `(&Graph) -> HashMap<usize, usize>` | Smallest-available color, vertex order |
| `dsatur_coloring` | `(&Graph) -> HashMap<usize, usize>` | Saturation-degree heuristic |
| `exact_coloring` | `(&Graph, max_colors) -> Option<HashMap<usize, usize>>` | Backtracking, finds χ(G) |
| `chromatic_number` | `(&HashMap<usize, usize>) -> usize` | Count distinct colors |

### `Graph`

| Method | Description |
|--------|-------------|
| `new()`, `add_edge(u, v)`, `add_node(u)` | Construction |
| `nodes()`, `neighbor_set(u)`, `node_count()`, `edge_count()` | Queries |

## Architecture Notes

This is a **γ (gamma)** module — pure combinatorial algorithms with deterministic output. In the γ + η = C framework, these coloring algorithms provide the assignment logic; an **η** layer would handle dynamic graph updates, distributed coloring, or integration with a register allocator's spill heuristics.

## References

- Karp, R. M. (1972). *Reducibility Among Combinatorial Problems*. In *Complexity of Computer Computations*, Plenum Press.
- Brélaz, D. (1979). *New Methods to Color the Vertices of a Graph*. Comm. ACM 22(4), 251–256.
- Appel, K. & Haken, W. (1976). *Every Planar Map is Four Colorable*. Bull. AMS 82(5).
- Chaitin, G. J. (1981). *Register Allocation and Spilling via Graph Coloring*. SIGPLAN Notices 17(6).

## License

MIT
