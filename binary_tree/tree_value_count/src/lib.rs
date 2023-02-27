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

/// This function takes in the root(which could be empty) of a binary tree
/// and a target value.
/// It should return the number of times that the
/// target occurs in the tree.
/// See recursive version below
pub fn tree_value_count(root: Option<TreeNodeRef>, target: i32) -> i32 {
    if root.is_none() {
        return 0;
    }
    let mut count = 0i32;

    // Do a breadth first search
    // starting with the root
    // and compare each node's `val`
    // with `target` and increment `count`
    let mut queue: VecDeque<TreeNodeRef> = VecDeque::new();
    let root = root.unwrap();
    queue.push_back(root);
    while !queue.is_empty() {
        let current: Rc<RefCell<TreeNode>> = queue.pop_front().unwrap();
        if current.borrow().val == target {
            count += 1;
        }

        if let Some(left) = &current.borrow().left {
            queue.push_back(left.clone());
        };
        if let Some(right) = &current.borrow().right {
            queue.push_back(right.clone());
        };
    }
    count
}

/// This is a recursive version of the
/// same logic
pub fn tree_value_count_recursive(
    root: Option<&TreeNodeRef>,
    target: i32,
) -> i32 {
    if let Some(root) = root {
        let count = if root.borrow().val == target { 1 } else { 0 };
        count
            + tree_value_count_recursive(root.borrow().left.as_ref(), target)
            + tree_value_count_recursive(root.borrow().right.as_ref(), target)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_value_count_00() {
        let mut node_a = TreeNode { val: 20, left: None, right: None };
        let mut node_b = TreeNode { val: 40, left: None, right: None };
        let mut node_c = TreeNode { val: 40, left: None, right: None };
        let node_d = TreeNode { val: 50, left: None, right: None };
        let node_e = TreeNode { val: 40, left: None, right: None };
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
            tree_value_count(Some(Rc::new(RefCell::new(node_a.clone()))), 40),
            3
        );
        assert_eq!(
            tree_value_count(Some(Rc::new(RefCell::new(node_a.clone()))), 60),
            0
        );
        assert_eq!(
            tree_value_count_recursive(
                Some(&Rc::new(RefCell::new(node_a))),
                40
            ),
            3
        );
    }

    #[test]
    fn test_tree_value_count_01() {
        let mut node_a = TreeNode { val: 20, left: None, right: None };
        let mut node_b = TreeNode { val: 30, left: None, right: None };
        let mut node_c = TreeNode { val: 40, left: None, right: None };
        let node_d = TreeNode { val: 50, left: None, right: None };
        let mut node_e = TreeNode { val: 60, left: None, right: None };
        let mut node_f = TreeNode { val: 70, left: None, right: None };
        let node_g = TreeNode { val: 80, left: None, right: None };
        let node_h = TreeNode { val: 80, left: None, right: None };
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
            tree_value_count(Some(Rc::new(RefCell::new(node_a.clone()))), 80),
            2
        );

        assert_eq!(
            tree_value_count_recursive(
                Some(&Rc::new(RefCell::new(node_a.clone()))),
                80
            ),
            2
        );
        assert_eq!(
            tree_value_count(Some(Rc::new(RefCell::new(node_a))), 85),
            0
        );
    }

    #[test]
    fn test_tree_value_count_02() {
        let node_a = TreeNode { val: 20, left: None, right: None };
        assert_eq!(
            tree_value_count(Some(Rc::new(RefCell::new(node_a.clone()))), 85),
            0
        );

        assert_eq!(
            tree_value_count(Some(Rc::new(RefCell::new(node_a.clone()))), 20),
            1
        );

        assert_eq!(
            tree_value_count_recursive(
                Some(&Rc::new(RefCell::new(node_a))),
                20
            ),
            1
        );
    }

    #[test]
    fn test_tree_value_count_03() {
        assert_eq!(tree_value_count(None, 5), 0);
        assert_eq!(tree_value_count_recursive(None, 5), 0);
    }

    #[test]
    fn test_tree_value_count_04() {
        let mut node_a = TreeNode { val: 20, left: None, right: None };
        let mut node_b = TreeNode { val: 30, left: None, right: None };
        let mut node_c = TreeNode { val: 40, left: None, right: None };
        let mut node_d = TreeNode { val: 70, left: None, right: None };
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
            tree_value_count(Some(Rc::new(RefCell::new(node_a.clone()))), 70),
            2
        );
        assert_eq!(
            tree_value_count_recursive(
                Some(&Rc::new(RefCell::new(node_a.clone()))),
                70
            ),
            2
        );
        assert_eq!(
            tree_value_count(Some(Rc::new(RefCell::new(node_a))), 60),
            1
        );
    }
}
