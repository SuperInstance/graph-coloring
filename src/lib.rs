//! Graph coloring algorithms: greedy, DSatur, and backtracking with pruning.

use std::collections::{HashMap, HashSet};

/// Simple adjacency-list graph.
#[derive(Debug, Clone)]
pub struct Graph {
    adj: HashMap<usize, HashSet<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Self { adj: HashMap::new() }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj.entry(u).or_default().insert(v);
        self.adj.entry(v).or_default().insert(u);
    }

    pub fn add_node(&mut self, u: usize) {
        self.adj.entry(u).or_default();
    }

    pub fn nodes(&self) -> Vec<usize> {
        let mut ns: Vec<usize> = self.adj.keys().copied().collect();
        ns.sort();
        ns
    }

    pub fn neighbor_set(&self, u: usize) -> Option<&HashSet<usize>> {
        self.adj.get(&u)
    }

    pub fn node_count(&self) -> usize {
        self.adj.len()
    }

    pub fn edge_count(&self) -> usize {
        self.adj.values().map(|s| s.len()).sum::<usize>() / 2
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

fn get_neighbors(graph: &Graph, u: usize) -> Vec<usize> {
    graph.neighbor_set(u).map(|s| s.iter().copied().collect()).unwrap_or_default()
}

fn get_neighbor_ref<'a>(graph: &'a Graph, u: usize) -> &'a HashSet<usize> {
    static EMPTY: std::sync::OnceLock<HashSet<usize>> = std::sync::OnceLock::new();
    graph.neighbor_set(u).unwrap_or_else(|| EMPTY.get_or_init(HashSet::new))
}

/// Greedy sequential coloring — O(V + E).
/// Colors nodes in order, assigning the smallest available color.
pub fn greedy_coloring(graph: &Graph) -> HashMap<usize, usize> {
    let mut coloring: HashMap<usize, usize> = HashMap::new();
    for node in graph.nodes() {
        let used: HashSet<usize> = get_neighbor_ref(graph, node)
            .iter()
            .filter_map(|&n| coloring.get(&n))
            .copied()
            .collect();
        let mut color = 0;
        while used.contains(&color) {
            color += 1;
        }
        coloring.insert(node, color);
    }
    coloring
}

/// DSatur (degree of saturation) coloring — better ordering heuristic.
/// At each step, picks the uncolored node with the highest saturation degree
/// (number of distinct colors among its neighbors), breaking ties by degree.
pub fn dsatur_coloring(graph: &Graph) -> HashMap<usize, usize> {
    let nodes = graph.nodes();
    let n = nodes.len();
    if n == 0 {
        return HashMap::new();
    }

    let mut coloring: HashMap<usize, usize> = HashMap::new();
    let mut saturation: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut colored: HashSet<usize> = HashSet::new();

    for &node in &nodes {
        saturation.insert(node, HashSet::new());
    }

    for _ in 0..n {
        let next = nodes
            .iter()
            .filter(|n| !colored.contains(n))
            .max_by(|&a, &b| {
                let sa = saturation[a].len();
                let sb = saturation[b].len();
                sa.cmp(&sb).then_with(|| {
                    get_neighbor_ref(graph, *a).len().cmp(&get_neighbor_ref(graph, *b).len())
                })
            })
            .unwrap();

        let used: HashSet<usize> = get_neighbor_ref(graph, *next)
            .iter()
            .filter_map(|&n| coloring.get(&n))
            .copied()
            .collect();

        let mut color = 0;
        while used.contains(&color) {
            color += 1;
        }
        coloring.insert(*next, color);
        colored.insert(*next);

        for &neighbor in get_neighbor_ref(graph, *next) {
            if !colored.contains(&neighbor) {
                saturation.get_mut(&neighbor).unwrap().insert(color);
            }
        }
    }

    coloring
}

/// Exact coloring via backtracking with pruning.
/// Finds the minimum chromatic number.
/// `max_colors` sets an upper bound (use greedy result + 1 as a starting point).
pub fn exact_coloring(graph: &Graph, max_colors: usize) -> Option<HashMap<usize, usize>> {
    let nodes = graph.nodes();
    if nodes.is_empty() {
        return Some(HashMap::new());
    }

    for num_colors in 1..=max_colors {
        let mut coloring = vec![None; *nodes.last().unwrap() + 1];
        if backtrack(graph, &nodes, 0, num_colors, &mut coloring) {
            return Some(
                nodes
                    .iter()
                    .map(|&n| (n, coloring[n].unwrap()))
                    .collect(),
            );
        }
    }
    None
}

fn backtrack(
    graph: &Graph,
    nodes: &[usize],
    idx: usize,
    num_colors: usize,
    coloring: &mut [Option<usize>],
) -> bool {
    if idx == nodes.len() {
        return true;
    }
    let node = nodes[idx];
    for color in 0..num_colors {
        let conflict = get_neighbor_ref(graph, node).iter().any(|&n| {
            n < coloring.len() && coloring[n] == Some(color)
        });
        if conflict {
            continue;
        }
        coloring[node] = Some(color);
        if backtrack(graph, nodes, idx + 1, num_colors, coloring) {
            return true;
        }
        coloring[node] = None;
    }
    false
}

/// Return the number of distinct colors used in a coloring.
pub fn chromatic_number(coloring: &HashMap<usize, usize>) -> usize {
    coloring.values().copied().max().map(|m| m + 1).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_triangle() -> Graph {
        let mut g = Graph::new();
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(0, 2);
        g
    }

    fn make_bipartite() -> Graph {
        let mut g = Graph::new();
        g.add_edge(0, 2);
        g.add_edge(0, 3);
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        g
    }

    #[test]
    fn test_greedy_triangle() {
        let g = make_triangle();
        let c = greedy_coloring(&g);
        assert_eq!(chromatic_number(&c), 3);
    }

    #[test]
    fn test_greedy_bipartite() {
        let g = make_bipartite();
        let c = greedy_coloring(&g);
        assert!(chromatic_number(&c) <= 2);
    }

    #[test]
    fn test_dsatur_triangle() {
        let g = make_triangle();
        let c = dsatur_coloring(&g);
        assert_eq!(chromatic_number(&c), 3);
    }

    #[test]
    fn test_dsatur_bipartite() {
        let g = make_bipartite();
        let c = dsatur_coloring(&g);
        assert_eq!(chromatic_number(&c), 2);
    }

    #[test]
    fn test_exact_triangle() {
        let g = make_triangle();
        let c = exact_coloring(&g, 3).unwrap();
        assert_eq!(chromatic_number(&c), 3);
    }

    #[test]
    fn test_exact_bipartite() {
        let g = make_bipartite();
        let c = exact_coloring(&g, 4).unwrap();
        assert_eq!(chromatic_number(&c), 2);
    }

    #[test]
    fn test_empty_graph() {
        let g = Graph::new();
        let c = greedy_coloring(&g);
        assert!(c.is_empty());
    }
}
