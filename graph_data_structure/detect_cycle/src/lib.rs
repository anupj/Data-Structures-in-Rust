use std::collections::HashMap;
use std::collections::HashSet;

/// This representation is vastly simpler
/// than the representation used elsewhere in
/// this codebase (look for the implementations
/// under the `graph_data_structure` folder in this
/// repo).
type Graph = HashMap<u32, Vec<u32>>;

/// Takes in an object representing the adjacency list
/// of a directed graph. The function should return
/// a boolean indicating whether or not the graph
/// contains a cycle.
fn is_cyclic(graph: &Graph) -> bool {
    let mut visited: HashSet<u32> = HashSet::new();
    let mut visiting: HashSet<u32> = HashSet::new();

    for node in graph.keys() {
        if detect_cycle(node, graph, &mut visited, &mut visiting) {
            return true;
        }
    }

    false
}

fn detect_cycle(
    node: &u32,
    graph: &Graph,
    visited: &mut HashSet<u32>,
    visiting: &mut HashSet<u32>,
) -> bool {
    if visiting.contains(node) {
        return true;
    }

    visiting.insert(*node);

    for neighbor in graph.get(node).unwrap() {
        if detect_cycle(neighbor, graph, visited, visiting) {
            return true;
        }
    }
    visited.insert(*node);
    visiting.remove(node);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_00() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(1, vec![2]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![1]);

        let detect_cycle = is_cyclic(&graph);
        assert_eq!(detect_cycle, true);
    }

    #[test]
    fn test_dfs_01() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(1, vec![2]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![4]);
        graph.insert(4, vec![]);

        let detect_cycle = is_cyclic(&graph);
        assert_eq!(detect_cycle, false);
    }

    #[test]
    fn test_dfs_02() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(1, vec![2, 3]);
        graph.insert(2, vec![]);
        graph.insert(3, vec![]);
        graph.insert(4, vec![5]);
        graph.insert(5, vec![4]);
        let detect_cycle = is_cyclic(&graph);
        assert_eq!(detect_cycle, true);
    }

    #[test]
    fn test_dfs_03() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(1, vec![2, 3]);
        graph.insert(2, vec![4, 5]);
        graph.insert(3, vec![]);
        graph.insert(4, vec![]);
        graph.insert(5, vec![]);
        graph.insert(6, vec![7]);
        graph.insert(7, vec![]);
        graph.insert(8, vec![7]);
        let detect_cycle = is_cyclic(&graph);
        assert_eq!(detect_cycle, false);
    }

    #[test]
    fn test_dfs_04() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(1, vec![2]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![1]);
        graph.insert(4, vec![]);

        let detect_cycle = is_cyclic(&graph);
        assert_eq!(detect_cycle, true);
    }
}
