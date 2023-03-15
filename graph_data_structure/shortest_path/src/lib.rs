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

/// Takes in an object representing the adjacency list of a undirected
/// possibly cyclic graph and two nodes (start, dst). The function
/// should return the length of the shortest path between A and B.
/// Consider the length as the number of edges in the path, not
/// the number of nodes. If there is no path between A and B,
/// then return -1.
/// Breadth-first search algorithm is better than Depth-first here.
fn shortest_path(graph: Graph, start_node: NodeId, dest_node: NodeId) -> i32 {
    let mut visited = HashSet::new();
    visited.insert(start_node);
    let mut queue: VecDeque<(NodeId, i32)> = VecDeque::new();
    queue.push_back((start_node, 0i32));

    while !queue.is_empty() {
        let (current_node, distance) = queue.pop_front().unwrap();
        if current_node == dest_node {
            return distance;
        }

        for neighbour in graph.adjacency_list.get(&current_node).unwrap() {
            if !visited.contains(neighbour) {
                visited.insert(*neighbour);
                queue.push_back((*neighbour, distance + 1));
            }
        }
    }
    return -1;
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
        graph.add_node(3);
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
        let result = shortest_path(graph.clone(), 0, 3);
        assert_eq!(result, 2);
        let result = shortest_path(graph.clone(), 0, 4);
        assert_eq!(result, -1);
        let result = shortest_path(graph, 3, 1);
        assert_eq!(result, 2);
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
        graph.add_node(7);
        graph.add_undirected_edge(0, 1);
        graph.add_undirected_edge(0, 2);
        graph.add_undirected_edge(1, 3);
        graph.add_undirected_edge(2, 4);
        graph.add_undirected_edge(3, 5);
        graph.add_undirected_edge(6, 7);
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
        // 6 <--> 7
        let result = shortest_path(graph.clone(), 0, 5);
        assert_eq!(result, 3);
        let result = shortest_path(graph.clone(), 0, 4);
        assert_eq!(result, 2);
        let result = shortest_path(graph.clone(), 2, 5);
        assert_eq!(result, 4);
        let result = shortest_path(graph.clone(), 3, 1);
        assert_eq!(result, 1);

        let result = shortest_path(graph, 3, 7);
        assert_eq!(result, -1);
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
        graph.add_undirected_edge(0, 1);
        graph.add_undirected_edge(0, 2);
        graph.add_undirected_edge(1, 3);
        graph.add_undirected_edge(2, 4);
        graph.add_undirected_edge(3, 5);
        graph.add_undirected_edge(4, 5);
        // 0 <--> 1
        // ^      ^
        // |      |
        // v      v
        // 2      3
        // ^      ^
        // |      |
        // v      v
        // 4 <--> 5
        //
        let result = shortest_path(graph.clone(), 0, 5);
        assert_eq!(result, 3);
        let result = shortest_path(graph.clone(), 0, 4);
        assert_eq!(result, 2);
        let result = shortest_path(graph.clone(), 2, 5);
        assert_eq!(result, 2);
        let result = shortest_path(graph.clone(), 1, 5);
        assert_eq!(result, 2);
        let result = shortest_path(graph.clone(), 4, 5);
        assert_eq!(result, 1);
    }
}
