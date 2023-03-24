use std::{collections::HashMap, usize};

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

/// This function takes in a number of courses (n) and a
/// list of prerequisites as arguments. Courses have ids
/// ranging from 0 through n - 1. A single prerequisite
/// of [A, B] means that course A must be taken before
/// course B. Return the minimum number of semesters
/// required to complete all n courses. There is no
/// limit on how many courses you can take in a
/// single semester, as long the prerequisites of
/// a course are satisfied before taking it.
///
/// Note that given prerequisite [A, B], you cannot
/// take course A and course B concurrently in the same
/// semester. You must take A in some semester before B.
///
/// You can assume that it is possible to eventually
/// complete all courses.
fn semesters_required(num_courses: usize, prereqs: &[[usize; 2]]) -> usize {
    let graph = build_graph(num_courses, prereqs);

    // This is a variation of the longest
    // path algorithm so we'll reuse this
    // algorithm from before
    longest_path(graph)
}

fn build_graph(num_courses: usize, prereqs: &[[usize; 2]]) -> Graph {
    let mut graph = Graph::new();
    for i in 0..num_courses {
        graph.add_node(i);
    }

    for prereq in prereqs {
        let a = prereq[0];
        let b = prereq[1];
        graph.add_directed_edge(a, b);
    }

    graph
}

/// This `fn` takes in an a directed acyclic graph. The
/// function should return the length of the longest path
/// within the graph. A path may start and end at any two
/// nodes. The length of a path is considered the number of
/// edges in the path, not the number of nodes.
/// Depth-first search algorithm
fn longest_path(graph: Graph) -> usize {
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
            distance.insert(*node, 1usize);
        }
    }

    for node in graph.adjacency_list.keys() {
        traverse_distance(&graph, *node, &mut distance);
    }

    // return the max value from the list
    // of values in distance since we want
    // the longest path
    *distance.values().max().unwrap_or(&1)
}

fn traverse_distance(
    graph: &Graph,
    current_node: NodeId,
    distance: &mut HashMap<usize, usize>,
) -> usize {
    // This is the base condition
    // If node is found the `distance`
    // map, then return its distance from a
    // terminal node.
    // It also acts as guard against visiting
    // nodes that have already been visited.
    if distance.contains_key(&current_node) {
        return *distance.get(&current_node).unwrap();
    }

    let mut max_distance = 0;
    for neighbour in graph.adjacency_list.get(&current_node).unwrap() {
        // lets call the function recursively for each neighbour
        // of this `current_node`
        let neighbour_distance =
            traverse_distance(graph, *neighbour, distance);
        if neighbour_distance > max_distance {
            max_distance = neighbour_distance;
        }
    }

    // Set the distance of the current node by
    // adding a 1 to max distance i.e. It is
    // an additional edge away from the current
    // distance to the terminal node
    distance.insert(current_node, 1 + max_distance);

    // return the new distance
    // This and above line could have been
    // written in one line as follows:
    // `distance.insert(current_node, 1 + max_distance).unwrap()`
    // I split it in two lines for clarity
    *distance.get(&current_node).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semester_required_00() {
        let num_courses = 6;
        let prereqs = [[1, 2], [2, 4], [3, 5], [0, 5]];
        let num_semeseters = semesters_required(num_courses, &prereqs);
        assert_eq!(num_semeseters, 3);
    }

    #[test]
    fn test_semester_required_01() {
        let num_courses = 7;
        let prereqs = [[4, 3], [3, 2], [2, 1], [1, 0], [5, 2], [5, 6]];
        let num_semeseters = semesters_required(num_courses, &prereqs);
        assert_eq!(num_semeseters, 5);
    }

    #[test]
    fn test_semester_required_02() {
        let num_courses = 5;
        let prereqs = [[1, 0], [3, 4], [1, 2], [3, 2]];
        let num_semeseters = semesters_required(num_courses, &prereqs);
        assert_eq!(num_semeseters, 2);
    }

    #[test]
    fn test_semester_required_03() {
        let num_courses = 12;
        let prereqs = [];
        let num_semeseters = semesters_required(num_courses, &prereqs);
        assert_eq!(num_semeseters, 1);
    }

    #[test]
    fn test_semester_required_04() {
        let num_courses = 3;
        let prereqs = [[0, 2], [0, 1], [1, 2]];
        let num_semeseters = semesters_required(num_courses, &prereqs);
        assert_eq!(num_semeseters, 3);
    }

    #[test]
    fn test_semester_required_05() {
        let num_courses = 6;
        let prereqs = [[3, 4], [3, 0], [3, 1], [3, 2], [3, 5]];
        let num_semeseters = semesters_required(num_courses, &prereqs);
        assert_eq!(num_semeseters, 2);
    }
}
