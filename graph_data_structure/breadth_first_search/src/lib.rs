use std::{
    collections::{HashMap, HashSet, VecDeque},
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

/// Breadth-first search algorithm
/// We cannot implement this recursively
/// so iterative approach it is
fn breadth_first_search(graph: Graph, start_node: NodeId) -> Vec<NodeId> {
    let mut visited = HashSet::new();
    let mut traversel = Vec::new();
    let mut queue = VecDeque::new();
    visited.insert(start_node);
    queue.push_back(start_node);
    while !queue.is_empty() {
        let current_node = queue.pop_front().unwrap();
        traversel.push(current_node);
        for neighbour in graph.adjacency_list.get(&current_node).unwrap() {
            if !visited.contains(neighbour) {
                visited.insert(*neighbour);
                queue.push_back(*neighbour);
            }
        }
    }
    traversel
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
        let traversal = breadth_first_search(graph.clone(), 0);
        assert_eq!(traversal, vec![0, 1, 2, 3]);
        let traversal = breadth_first_search(graph, 3);
        assert_eq!(traversal, vec![3]);
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
        let traversal = breadth_first_search(graph.clone(), 0);
        assert_eq!(traversal, vec![0, 1, 2, 3, 4, 5]);
        let traversal = breadth_first_search(graph, 3);
        assert_eq!(traversal, vec![3, 5]);
    }
}
