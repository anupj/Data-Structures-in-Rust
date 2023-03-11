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
        Self { adjacency_list: HashMap::new() }
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

/// takes in an object representing the adjacency list of a directed acyclic
/// graph and two nodes (src, dst). The function should return a boolean
/// indicating whether or not there exists a directed path between the source
/// and destination nodes.
/// Depth-first search algorithm
fn has_path(graph: Graph, start_node: NodeId, dest_node: NodeId) -> bool {
    let mut visited = HashSet::new();
    has_path_helper(&graph, start_node, dest_node, &mut visited)
}

fn has_path_helper(
    graph: &Graph,
    current_node: NodeId,
    dest_node: NodeId,
    visited: &mut HashSet<NodeId>,
) -> bool {
    if current_node == dest_node {
        return true;
    }
    visited.insert(current_node);
    for neighbour in graph.adjacency_list.get(&current_node).unwrap() {
        if !visited.contains(neighbour)
            && has_path_helper(graph, *neighbour, dest_node, visited)
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut graph = Graph::new();
        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        assert_eq!(graph.adjacency_list.len(), 4);
        assert!(graph.adjacency_list.contains_key(&0));
        assert!(graph.adjacency_list.contains_key(&1));
        assert!(graph.adjacency_list.contains_key(&2));
        assert!(graph.adjacency_list.contains_key(&3));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();
        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_directed_edge(0, 1);
        graph.add_directed_edge(0, 2);
        graph.add_directed_edge(1, 2);
        graph.add_directed_edge(2, 3);
        assert_eq!(graph.adjacency_list.get(&0), Some(&vec![1, 2]));
        assert_eq!(graph.adjacency_list.get(&1), Some(&vec![2]));
        assert_eq!(graph.adjacency_list.get(&2), Some(&vec![3]));
    }

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
        let result = has_path(graph.clone(), 0, 3);
        assert_eq!(result, true);
        let result = has_path(graph, 3, 1);
        assert_eq!(result, false);
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
        let result = has_path(graph.clone(), 0, 5);
        assert_eq!(result, true);
        let result = has_path(graph.clone(), 0, 4);
        assert_eq!(result, true);
        let result = has_path(graph.clone(), 2, 5);
        assert_eq!(result, false);
        let result = has_path(graph, 3, 1);
        assert_eq!(result, false);
    }
}
