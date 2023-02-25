use std::{cell::RefCell, cmp, rc::Rc};

type TreeNodeRef = Rc<RefCell<TreeNode>>;
/// Represents a binary tree
/// The criteria for binary tree
/// a) has at most 2 children
/// b) exactly 1 root
/// c) exactly 1 path between root
///    and any node
#[derive(Debug, Clone)]
pub struct TreeNode {
    // we are not going to use
    // `val` for this algorithm
    _val: i32,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

/// This function takes in the root of a binary tree.
/// It should return a number representing the
/// height of the tree.
/// The height of a binary tree is defined as the
/// maximal number of edges from the root node to any leaf node.
/// If the tree is empty, return -1.
///
/// Time: O(n)
/// Space: O(n)
pub fn tree_height(root: Option<&TreeNodeRef>) -> i32 {
    if let Some(root) = root {
        let left_height = tree_height(root.borrow().left.as_ref());
        let right_height = tree_height(root.borrow().right.as_ref());
        return 1 + cmp::max(left_height, right_height);
    }
    const EMPTY_TREE: i32 = -1;
    EMPTY_TREE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_height_values_00() {
        let mut node_a = TreeNode { _val: 20, left: None, right: None };
        let mut node_b = TreeNode { _val: 30, left: None, right: None };
        let mut node_c = TreeNode { _val: 40, left: None, right: None };
        let node_d = TreeNode { _val: 50, left: None, right: None };
        let node_e = TreeNode { _val: 60, left: None, right: None };
        let node_f = TreeNode { _val: 70, left: None, right: None };

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
        assert_eq!(tree_height(Some(&Rc::new(RefCell::new(node_a)))), 2);
    }

    #[test]
    fn test_tree_height_values_01() {
        let mut node_a = TreeNode { _val: 20, left: None, right: None };
        let mut node_b = TreeNode { _val: 30, left: None, right: None };
        let mut node_c = TreeNode { _val: 40, left: None, right: None };
        let node_d = TreeNode { _val: 50, left: None, right: None };
        let mut node_e = TreeNode { _val: 60, left: None, right: None };
        let mut node_f = TreeNode { _val: 70, left: None, right: None };
        let node_g = TreeNode { _val: 80, left: None, right: None };
        let node_h = TreeNode { _val: 90, left: None, right: None };
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
        assert_eq!(tree_height(Some(&Rc::new(RefCell::new(node_a)))), 3);
    }

    #[test]
    fn test_tree_height_values_02() {
        let node_a = TreeNode { _val: 20, left: None, right: None };
        assert_eq!(tree_height(Some(&Rc::new(RefCell::new(node_a)))), 0);
    }

    #[test]
    fn test_tree_height_values_03() {
        let mut node_a = TreeNode { _val: 20, left: None, right: None };
        let mut node_b = TreeNode { _val: 30, left: None, right: None };
        let mut node_c = TreeNode { _val: 40, left: None, right: None };
        let mut node_d = TreeNode { _val: 50, left: None, right: None };
        let node_e = TreeNode { _val: 60, left: None, right: None };

        let node_x = TreeNode { _val: 70, left: None, right: None };
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
        assert_eq!(tree_height(Some(&Rc::new(RefCell::new(node_a)))), 4);
    }

    #[test]
    fn test_tree_height_values_04() {
        assert_eq!(tree_height(None), -1);
    }
}
