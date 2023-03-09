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

/// Depth-first search algorithm
fn depth_first_search(graph: Graph, start_node: NodeId) -> Vec<NodeId> {
    let mut visited = HashSet::new();
    let mut traversel = Vec::new();
    dfs_helper(&graph, start_node, &mut visited, &mut traversel);
    traversel
}

fn dfs_helper(
    graph: &Graph,
    current_node: NodeId,
    visited: &mut HashSet<NodeId>,
    traversel: &mut Vec<NodeId>,
) {
    visited.insert(current_node);
    traversel.push(current_node);
    for neighbour in graph.adjacency_list.get(&current_node).unwrap() {
        if !visited.contains(neighbour) {
            dfs_helper(&graph, *neighbour, visited, traversel);
        }
    }
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
    fn test_dfs() {
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
        let traversal = depth_first_search(graph.clone(), 0);
        assert_eq!(traversal, vec![0, 1, 2, 3]);
        let traversal = depth_first_search(graph, 3);
        assert_eq!(traversal, vec![3]);
    }
}
