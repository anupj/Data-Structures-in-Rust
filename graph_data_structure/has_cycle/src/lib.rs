use std::{
    collections::{HashMap, HashSet},
    usize,
};

pub type NodeId = usize;

#[derive(Clone, Debug)]
pub struct Graph {
    adjacency_list: HashMap<NodeId, Vec<NodeId>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_node(&mut self, node_id: NodeId) {
        self.adjacency_list.entry(node_id).or_insert(Vec::new());
    }

    // Use this method to add directed edge u --> v
    fn add_directed_edge(&mut self, u: NodeId, v: NodeId) {
        self.adjacency_list.entry(u).or_insert(Vec::new()).push(v);
    }

    // add a bi-directional edge u <--> v
    fn add_undirected_edge(&mut self, u: NodeId, v: NodeId) {
        self.adjacency_list.entry(u).or_insert(Vec::new()).push(v);
        self.adjacency_list.entry(v).or_insert(Vec::new()).push(u);
    }
}

/// This function takes in an object representing
/// the adjacency list of a directed graph. The
/// function should return a boolean indicating whether
/// or not the graph contains a cycle.
///
/// white-grey-black cycle detection algorithm
fn has_cycle(graph: Graph) -> bool {
    // white means not visited yet (default)
    let mut visiting = HashSet::new(); // grey
    let mut visited = HashSet::new(); // black

    for start_node in &graph.adjacency_list {
        if detect_cycle(&graph, *start_node.0, &mut visiting, &mut visited) {
            return true;
        }
    }
    false
}

fn detect_cycle(
    graph: &Graph,
    current_node: NodeId,
    visiting: &mut HashSet<NodeId>,
    visited: &mut HashSet<NodeId>,
) -> bool {
    // I've visited this node before
    // so no need to go through it again
    // thus breaking the recursion
    if visited.contains(&current_node) {
        return false;
    };

    // If I find this node in `visiting` means
    // I've traversed this node before and
    // therefore I've come back to this node
    // hence a cycle
    if visiting.contains(&current_node) {
        return true;
    }

    visiting.insert(current_node);

    for neighbour in graph.adjacency_list.get(&current_node).unwrap() {
        if detect_cycle(graph, *neighbour, visiting, visited) {
            return true;
        }
    }

    // We have processed `current_node`
    // so we are no longer `visiting` it
    // but it is now `visited`
    visiting.remove(&current_node);
    visited.insert(current_node);

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_00() {
        let mut graph = Graph::new();
        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_directed_edge(0, 1);
        graph.add_directed_edge(0, 2);
        graph.add_directed_edge(1, 2);
        graph.add_directed_edge(2, 3);
        // 0 ---> 1
        // |      |
        // v      |
        // 2 <----
        // |
        // v
        // 3
        let contains_cycle = has_cycle(graph);
        assert_eq!(contains_cycle, false);
    }

    #[test]
    fn test_dfs_01() {
        let mut graph = Graph::new();
        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_node(4);
        graph.add_node(5);
        graph.add_directed_edge(0, 1);
        graph.add_directed_edge(0, 2);
        graph.add_directed_edge(1, 3);
        graph.add_directed_edge(2, 4);
        graph.add_directed_edge(3, 5);
        // 0 ---> 1
        // |      |
        // v      v
        // 2      3
        // |      |
        // v      v
        // 4      5
        let contains_cycle = has_cycle(graph);
        assert_eq!(contains_cycle, false);
    }

    #[test]
    fn test_dfs_02() {
        let mut graph = Graph::new();
        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_directed_edge(0, 1);
        graph.add_directed_edge(1, 2);
        graph.add_directed_edge(2, 0);
        // 0 ---> 1
        // ^      |
        // |      |
        // 2 <--- v
        let contains_cycle = has_cycle(graph);
        assert_eq!(contains_cycle, true);
    }

    #[test]
    fn test_dfs_03() {
        let mut graph = Graph::new();
        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_directed_edge(0, 1);
        graph.add_directed_edge(0, 2);
        graph.add_directed_edge(1, 2);
        graph.add_directed_edge(2, 3);
        // 0 ---> 1
        // |      |
        // v      |
        // 2 <--- v
        // |
        // v
        // 3
        let contains_cycle = has_cycle(graph);
        assert_eq!(contains_cycle, false);
    }
    #[test]
    fn test_dfs_04() {
        let mut graph = Graph::new();
        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_node(4);
        graph.add_node(5);
        graph.add_directed_edge(0, 1);
        graph.add_directed_edge(0, 2);
        graph.add_directed_edge(4, 5);
        graph.add_directed_edge(5, 4);
        // 0 ---> 1
        // ^
        // |
        // 2
        //
        // 4 <--> 5
        let contains_cycle = has_cycle(graph);
        assert_eq!(contains_cycle, true);
    }
    #[test]
    fn test_dfs_05() {
        let mut graph = Graph::new();
        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_node(4);
        graph.add_node(5);
        graph.add_node(6);
        graph.add_node(7);
        graph.add_directed_edge(0, 1);
        graph.add_directed_edge(0, 2);
        graph.add_directed_edge(1, 3);
        graph.add_directed_edge(1, 4);
        graph.add_directed_edge(5, 6);
        graph.add_directed_edge(7, 6);
        // 0 ---> 1 ---> 3
        // |      |
        // v      v
        // 2      4
        //
        // 5 --> 6 <-- 7
        let contains_cycle = has_cycle(graph);
        assert_eq!(contains_cycle, false);
    }

    #[test]
    fn test_dfs_06() {
        let mut graph = Graph::new();
        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(12);
        graph.add_directed_edge(0, 1);
        graph.add_directed_edge(1, 2);
        graph.add_directed_edge(2, 0);
        // 0 ---> 1
        // ^      |
        // |      |
        // 2 <--- v
        //
        // 12
        let contains_cycle = has_cycle(graph);
        assert_eq!(contains_cycle, true);
    }

    #[test]
    fn test_dfs_07() {
        let mut graph = Graph::new();
        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_directed_edge(0, 1);
        graph.add_directed_edge(1, 2);
        graph.add_directed_edge(2, 3);
        graph.add_directed_edge(3, 1);
        // 0 ---> 1 --> 2
        //        ^     |
        //        |     |
        //        3 <-- v
        let contains_cycle = has_cycle(graph);
        assert_eq!(contains_cycle, true);
    }
}
