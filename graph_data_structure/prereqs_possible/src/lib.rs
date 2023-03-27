use std::collections::HashMap;
use std::collections::HashSet;

/// This representation is vastly simpler
/// than the representation used elsewhere in
/// this codebase (look for the implementations
/// under the `graph_data_structure` folder in this
/// repo).
type Graph = HashMap<u32, Vec<u32>>;

/// Takes in a number of courses (n) and
/// prerequisites as arguments. Courses have ids
/// ranging from 0 through n - 1. A single
/// prerequisite of [A, B] means that course A
/// must be taken before course B. The function
/// should return a boolean indicating whether or
/// not it is possible to complete all courses.
fn prereqs_possible(num_courses: u32, prereqs: &[[u32; 2]]) -> bool {
    let mut graph = build_graph(num_courses, prereqs);

    let mut visited: HashSet<u32> = HashSet::new();
    let mut visiting: HashSet<u32> = HashSet::new();

    for node in graph.keys() {
        if detect_cycle(node, &graph, &mut visited, &mut visiting) {
            return false;
        }
    }

    true
}

fn build_graph(num_courses: u32, prereqs: &[[u32; 2]]) -> Graph {
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    for i in 0..num_courses {
        graph.insert(i, Vec::new());
    }

    for prereq in prereqs {
        let course_a = prereq[0];
        let course_b = prereq[1];
        graph.entry(course_a).or_insert(Vec::new()).push(course_b);
    }
    graph
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
        let num_courses = 6;
        let prereqs = [[0, 1], [2, 3], [0, 2], [1, 3], [4, 5]];
        let result = prereqs_possible(num_courses, &prereqs);
        assert_eq!(result, true);
    }

    #[test]
    fn test_dfs_01() {
        let num_courses = 6;
        let prereqs = [[0, 1], [2, 3], [0, 2], [1, 3], [4, 5], [3, 0]];
        let result = prereqs_possible(num_courses, &prereqs);
        assert_eq!(result, false);
    }

    #[test]
    fn test_dfs_02() {
        let num_courses = 5;
        let prereqs = [[2, 4], [1, 0], [0, 2], [0, 4]];
        let result = prereqs_possible(num_courses, &prereqs);
        assert_eq!(result, true);
    }

    #[test]
    fn test_dfs_03() {
        let num_courses = 6;
        let prereqs = [[2, 4], [1, 0], [0, 2], [0, 4], [5, 3], [3, 5]];
        let result = prereqs_possible(num_courses, &prereqs);
        assert_eq!(result, false);
    }
    #[test]
    fn test_dfs_04() {
        let num_courses = 8;
        let prereqs = [[1, 0], [0, 6], [2, 0], [0, 5], [3, 7], [4, 3]];
        let result = prereqs_possible(num_courses, &prereqs);
        assert_eq!(result, true);
    }

    #[test]
    fn test_dfs_05() {
        let num_courses = 8;
        let prereqs = [[1, 0], [0, 6], [2, 0], [0, 5], [3, 7], [7, 4], [4, 3]];
        let result = prereqs_possible(num_courses, &prereqs);
        assert_eq!(result, false);
    }

    #[test]
    fn test_dfs_06() {
        let num_courses = 42;
        let prereqs = [[6, 36]];
        let result = prereqs_possible(num_courses, &prereqs);
        assert_eq!(result, true);
    }
}
