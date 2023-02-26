use std::{cell::RefCell, collections::VecDeque, rc::Rc};

type TreeNodeRef = Rc<RefCell<TreeNode>>;
/// Represents a binary tree
/// The criteria for binary tree
/// a) has at most 2 children
/// b) exactly 1 root
/// c) exactly 1 path between root
///    and any node
#[derive(Debug, Clone)]
pub struct TreeNode {
    val: i32,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

/// This function takes in the root of a binary tree
/// that contains number values.
/// It should return an array containing the average
/// value of each level.
///
///  input tree
///      20
///    /    \
///   30     40
///  / \      \
/// 50  60    70
///
/// Output
/// [ 20, 35, 60]
///
/// Time: O(n)
/// Space: O(n)
pub fn tree_level_averages(root: Option<TreeNodeRef>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }

    let mut levels = vec![vec![]];

    #[derive(Debug, Clone)]
    struct CurrentNode {
        node: TreeNodeRef,
        node_level_number: usize,
    }

    let mut queue: VecDeque<CurrentNode> = VecDeque::new();
    queue.push_back(CurrentNode { node: root.unwrap(), node_level_number: 0 });

    while !queue.is_empty() {
        let current_node = queue.pop_front().unwrap();

        let level_number = current_node.node_level_number;
        let node = &current_node.node;
        let val: i32 = node.borrow().val;

        // This is the core of the logic for this
        // algorithm to work.
        // Insert into the 2 dimensional array (`vec`), the
        // values v[n][m] where n is the level number, and
        // m is the value a that level
        if levels.len() == level_number {
            levels.insert(level_number, vec![val]);
        } else {
            levels[level_number].push(val);
        }

        if let Some(ref left_node) = node.borrow().left {
            queue.push_back(CurrentNode {
                node: left_node.clone(),
                node_level_number: level_number + 1,
            });
        };
        if let Some(ref right_node) = node.borrow().right {
            queue.push_back(CurrentNode {
                node: right_node.clone(),
                node_level_number: level_number + 1,
            });
        };
    }

    let mut result: Vec<i32> = Vec::new();

    for level in levels {
        result.push(avg(level));
    }

    result
}

fn avg(list_of_nums: Vec<i32>) -> i32 {
    let sum: i32 = list_of_nums.iter().sum();
    let average = sum / list_of_nums.len() as i32;
    average
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_level_averages_values_00() {
        let mut node_a = TreeNode { val: 20, left: None, right: None };
        let mut node_b = TreeNode { val: 30, left: None, right: None };
        let mut node_c = TreeNode { val: 40, left: None, right: None };
        let node_d = TreeNode { val: 50, left: None, right: None };
        let node_e = TreeNode { val: 60, left: None, right: None };
        let node_f = TreeNode { val: 70, left: None, right: None };

        //      a
        //    /   \
        //   b     c
        //  / \     \
        // d   e     f
        node_b.left = Some(Rc::new(RefCell::new(node_d)));
        node_b.right = Some(Rc::new(RefCell::new(node_e)));
        node_c.right = Some(Rc::new(RefCell::new(node_f)));
        node_a.left = Some(Rc::new(RefCell::new(node_b)));
        node_a.right = Some(Rc::new(RefCell::new(node_c)));

        assert_eq!(
            tree_level_averages(Some(Rc::new(RefCell::new(node_a)))),
            vec![20, 35, 60]
        );
    }

    #[test]
    fn test_tree_level_averages_values_01() {
        let mut node_a = TreeNode { val: 20, left: None, right: None };
        let mut node_b = TreeNode { val: 30, left: None, right: None };
        let mut node_c = TreeNode { val: 40, left: None, right: None };
        let node_d = TreeNode { val: 50, left: None, right: None };
        let mut node_e = TreeNode { val: 60, left: None, right: None };
        let mut node_f = TreeNode { val: 70, left: None, right: None };
        let node_g = TreeNode { val: 80, left: None, right: None };
        let node_h = TreeNode { val: 90, left: None, right: None };
        //      a
        //    /   \
        //   b     c
        //  / \     \
        // d   e     f
        //    /       \
        //   g         h
        node_b.left = Some(Rc::new(RefCell::new(node_d)));
        node_e.left = Some(Rc::new(RefCell::new(node_g)));
        node_b.right = Some(Rc::new(RefCell::new(node_e)));
        node_f.right = Some(Rc::new(RefCell::new(node_h)));

        node_c.right = Some(Rc::new(RefCell::new(node_f)));
        node_a.left = Some(Rc::new(RefCell::new(node_b)));
        node_a.right = Some(Rc::new(RefCell::new(node_c)));
        assert_eq!(
            tree_level_averages(Some(Rc::new(RefCell::new(node_a)))),
            vec![20, 35, 60, 85]
        );
    }

    #[test]
    fn test_tree_level_averages_values_02() {
        let node_a = TreeNode { val: 20, left: None, right: None };
        assert_eq!(
            tree_level_averages(Some(Rc::new(RefCell::new(node_a)))),
            vec![20]
        );
    }

    #[test]
    fn test_tree_level_averages_values_03() {
        let mut node_a = TreeNode { val: 20, left: None, right: None };
        let mut node_b = TreeNode { val: 30, left: None, right: None };
        let mut node_c = TreeNode { val: 40, left: None, right: None };
        let mut node_d = TreeNode { val: 50, left: None, right: None };
        let node_e = TreeNode { val: 60, left: None, right: None };

        let node_x = TreeNode { val: 70, left: None, right: None };
        //      a
        //       \
        //        b
        //       /
        //      c
        //     / \
        //    x   d
        //         \
        //          e
        node_d.right = Some(Rc::new(RefCell::new(node_e)));
        node_c.right = Some(Rc::new(RefCell::new(node_d)));
        node_c.left = Some(Rc::new(RefCell::new(node_x)));
        node_b.left = Some(Rc::new(RefCell::new(node_c)));

        node_a.right = Some(Rc::new(RefCell::new(node_b)));
        assert_eq!(
            tree_level_averages(Some(Rc::new(RefCell::new(node_a)))),
            vec![20, 30, 40, 60, 60]
        );
    }

    #[test]
    fn test_tree_level_averages_values_04() {
        // empty output test
        assert_eq!(tree_level_averages(None), vec![]);
    }
}
