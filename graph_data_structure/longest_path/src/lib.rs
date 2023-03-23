use std::{collections::HashMap, usize};

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
/// Takes in an adjacency list for a directed acyclic graph. The
/// function should return the length of the longest path
/// within the graph. A path may start and end at any two
/// nodes. The length of a path is considered the number of
/// edges in the path, not the number of nodes.
/// Depth-first search algorithm
fn longest_path(graph: Graph) -> u32 {
    // The `key` here is the node
    // and `value` is the distance of the node
    // from a terminal node (i.e. a node without any
    // outgoing edge)
    //  A --> B --> C --> D
    //              |    ^
    //              v    |
    //              F ---
    //  distance = (A, 3) i.e A is 3 edges away from D
    //  distance = (C, 1), (F, 1) and so on
    let mut distance = HashMap::new();

    for node in graph.adjacency_list.keys() {
        if graph.adjacency_list.get(node).unwrap().is_empty() {
            distance.insert(*node, 0u32);
        }
    }

    for node in graph.adjacency_list.keys() {
        traverse_distance(&graph, *node, &mut distance);
    }

    // traverse_distance(&graph, start_node, &mut distance, &mut traversel);
    *distance.values().max().unwrap_or(&0)
}

fn traverse_distance(
    graph: &Graph,
    current_node: NodeId,
    distance: &mut HashMap<usize, u32>,
) -> u32 {
    // This is the base condition
    // If node is found the `distance`
    // map, then return its distance from a
    // terminal node.
    if distance.contains_key(&current_node) {
        return *distance.get(&current_node).unwrap();
    }

    let mut max_distance = 0;
    for neighbour in graph.adjacency_list.get(&current_node).unwrap() {
        let attempt = traverse_distance(graph, *neighbour, distance);
        if attempt > max_distance {
            max_distance = attempt;
        }
    }
    distance.insert(current_node, 1 + max_distance);
    *distance.get(&current_node).unwrap()
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
        let distance = longest_path(graph);
        assert_eq!(distance, 3);
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
        let distance = longest_path(graph);
        assert_eq!(distance, 3);
    }

    #[test]
    fn test_dfs_02() {
        let mut graph = Graph::new();
        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(10);
        graph.add_node(11);
        graph.add_node(12);
        graph.add_node(13);
        graph.add_node(14);
        graph.add_directed_edge(0, 1);
        graph.add_directed_edge(0, 2);
        graph.add_directed_edge(1, 2);
        graph.add_directed_edge(10, 11);
        graph.add_directed_edge(11, 12);
        graph.add_directed_edge(11, 13);
        graph.add_directed_edge(11, 14);
        graph.add_directed_edge(12, 13);
        graph.add_directed_edge(13, 14);
        // 0 ---> 1
        // |      |
        // v      |
        // 2 <----
        //
        //        12
        //        ^----
        //        |    |
        //        |    v
        // 10--->11--->13
        //        |    |
        //        |    |
        //        v    |
        //        14 <-
        let distance = longest_path(graph);
        assert_eq!(distance, 4);
    }
}
