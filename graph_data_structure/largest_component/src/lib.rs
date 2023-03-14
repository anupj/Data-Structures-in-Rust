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

/// Takes in an undirected graph and returns
/// the size of the largest connected component
/// in the graph.
fn largest_component(graph: Graph) -> u32 {
    let mut visited = HashSet::new();
    let mut largest = 0u32;
    for node in graph.adjacency_list.keys() {
        let size = explore_size(&graph, *node, &mut visited);
        if size > largest {
            largest = size;
        }
    }

    largest
}

/// traverse/explore the graph with the
/// current node and return once
/// traversed successfully.
/// Track the visited nodes so that we don't
/// explore them again when called
/// again by `connected_components_count`
/// or via the recursive call.
fn explore_size(
    graph: &Graph,
    current_node: NodeId,
    visited: &mut HashSet<NodeId>,
) -> u32 {
    if visited.contains(&current_node) {
        return 0u32;
    }
    visited.insert(current_node);

    let mut size = 1u32;
    for neighbour in graph.adjacency_list.get(&current_node).unwrap() {
        size += explore_size(graph, *neighbour, visited);
    }
    size
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
        graph.add_undirected_edge(0, 1);
        graph.add_undirected_edge(0, 2);
        graph.add_undirected_edge(1, 2);
        graph.add_undirected_edge(2, 3);
        assert_eq!(graph.adjacency_list.get(&0), Some(&vec![1, 2]));
        assert_eq!(graph.adjacency_list.get(&1), Some(&vec![0, 2]));
        assert_eq!(graph.adjacency_list.get(&2), Some(&vec![0, 1, 3]));
    }

    #[test]
    fn test_dfs_00() {
        let mut graph = Graph::new();
        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_node(4);
        graph.add_node(5);
        graph.add_undirected_edge(0, 1);
        graph.add_undirected_edge(0, 2);
        graph.add_undirected_edge(1, 2);
        graph.add_undirected_edge(2, 3);
        graph.add_undirected_edge(4, 5);
        // 0 ---> 1
        // ^      ^
        // |      |
        // v      |
        // 2 <----
        // ^
        // |
        // v
        // 3
        //
        // 4 <--> 5
        let count = largest_component(graph);
        assert_eq!(count, 4);
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
        graph.add_node(6);
        graph.add_undirected_edge(0, 1);
        graph.add_undirected_edge(0, 2);
        graph.add_undirected_edge(1, 3);
        graph.add_undirected_edge(2, 4);
        graph.add_undirected_edge(3, 5);
        // 0 ---> 1
        // ^      ^
        // |      |
        // v      v
        // 2      3
        // ^      ^
        // |      |
        // v      v
        // 4      5
        //
        // 6
        let count = largest_component(graph);
        assert_eq!(count, 6);
    }

    #[test]
    fn test_dfs_02() {
        let mut graph = Graph::new();
        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_node(4);
        graph.add_node(5);
        graph.add_node(6);
        graph.add_node(7);
        graph.add_node(8);
        graph.add_undirected_edge(0, 1);
        graph.add_undirected_edge(0, 2);
        graph.add_undirected_edge(1, 3);
        graph.add_undirected_edge(2, 4);
        graph.add_undirected_edge(3, 5);
        graph.add_undirected_edge(7, 8);
        // 0 ---> 1
        // ^      ^
        // |      |
        // v      v
        // 2      3
        // ^      ^
        // |      |
        // v      v
        // 4      5
        //
        // 6
        //
        // 7 <--> 8
        let count = largest_component(graph);
        assert_eq!(count, 6);
    }
}
