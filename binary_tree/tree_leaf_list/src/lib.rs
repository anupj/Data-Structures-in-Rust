use std::{cell::RefCell, rc::Rc};

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
/// returns an array containing all values
/// of all leaf nodes in left-to-right order.
pub fn tree_leaf_list(root: TreeNodeRef) -> Vec<i32> {
    let mut leaves = Vec::new();
    let mut stack = vec![root];
    while !stack.is_empty() {
        let current: Rc<RefCell<TreeNode>> = stack.pop().unwrap();
        if current.borrow().left.is_none() && current.borrow().right.is_none()
        {
            leaves.push(current.borrow().val);
        }

        // `Rc.clone()` is cheap
        if let Some(right) = &current.borrow().right {
            stack.push(right.clone());
        };

        if let Some(left) = &current.borrow().left {
            stack.push(left.clone());
        };
    }
    leaves
}

/// Recursive approach
/// WARNING: Here be 🐉
/// Time: O(n)
/// Space: O(n)
pub fn leaf_list_recursive(root: TreeNodeRef) -> Vec<i32> {
    let mut leaves = Vec::new();
    fill_leaves(Some(&root), &mut leaves);
    leaves
}

// This is where all the recursive fun happens
pub fn fill_leaves(root: Option<&TreeNodeRef>, leaves: &mut Vec<i32>) {
    // Check if `root` has `Some`thing
    if let Some(root) = root {
        if root.borrow().left.is_none() && root.borrow().right.is_none() {
            leaves.push(root.borrow().val);
        }

        fill_leaves(root.borrow().left.as_ref(), leaves);
        fill_leaves(root.borrow().right.as_ref(), leaves);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_leaf_list_00() {
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
            tree_leaf_list(Rc::new(RefCell::new(node_a))),
            &[50, 60, 70]
        );
    }

    #[test]
    fn test_tree_leaf_list_recursive_00() {
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
            leaf_list_recursive(Rc::new(RefCell::new(node_a))),
            &[50, 60, 70]
        );
    }

    #[test]
    fn test_tree_leaf_list_01() {
        let mut node_a = TreeNode { val: 20, left: None, right: None };
        let mut node_b = TreeNode { val: 30, left: None, right: None };
        let mut node_c = TreeNode { val: 40, left: None, right: None };
        let node_d = TreeNode { val: 50, left: None, right: None };
        let mut node_e = TreeNode { val: 60, left: None, right: None };
        let node_f = TreeNode { val: 70, left: None, right: None };
        let node_g = TreeNode { val: 80, left: None, right: None };

        //      a
        //    /   \
        //   b     c
        //  / \     \
        // d   e     f
        //    /
        //   g
        node_b.left = Some(Rc::new(RefCell::new(node_d)));
        node_e.left = Some(Rc::new(RefCell::new(node_g)));
        node_b.right = Some(Rc::new(RefCell::new(node_e)));
        node_c.right = Some(Rc::new(RefCell::new(node_f)));
        node_a.left = Some(Rc::new(RefCell::new(node_b)));
        node_a.right = Some(Rc::new(RefCell::new(node_c)));
        assert_eq!(
            tree_leaf_list(Rc::new(RefCell::new(node_a))),
            &[50, 80, 70]
        );
    }
    #[test]
    fn test_tree_leaf_list_recursive_01() {
        let mut node_a = TreeNode { val: 20, left: None, right: None };
        let mut node_b = TreeNode { val: 30, left: None, right: None };
        let mut node_c = TreeNode { val: 40, left: None, right: None };
        let node_d = TreeNode { val: 50, left: None, right: None };
        let mut node_e = TreeNode { val: 60, left: None, right: None };
        let node_f = TreeNode { val: 70, left: None, right: None };
        let node_g = TreeNode { val: 80, left: None, right: None };

        //      a
        //    /   \
        //   b     c
        //  / \     \
        // d   e     f
        //    /
        //   g
        node_b.left = Some(Rc::new(RefCell::new(node_d)));
        node_e.left = Some(Rc::new(RefCell::new(node_g)));
        node_b.right = Some(Rc::new(RefCell::new(node_e)));
        node_c.right = Some(Rc::new(RefCell::new(node_f)));
        node_a.left = Some(Rc::new(RefCell::new(node_b)));
        node_a.right = Some(Rc::new(RefCell::new(node_c)));
        assert_eq!(
            leaf_list_recursive(Rc::new(RefCell::new(node_a))),
            &[50, 80, 70]
        );
    }

    #[test]
    fn test_tree_leaf_list_02() {
        let node_a = TreeNode { val: 20, left: None, right: None };
        assert_eq!(tree_leaf_list(Rc::new(RefCell::new(node_a))), &[20]);
    }

    #[test]
    fn test_tree_leaf_list_recursive_02() {
        let node_a = TreeNode { val: 20, left: None, right: None };
        assert_eq!(leaf_list_recursive(Rc::new(RefCell::new(node_a))), &[20]);
    }

    #[test]
    fn test_tree_leaf_list_03() {
        let mut node_a = TreeNode { val: 20, left: None, right: None };
        let mut node_b = TreeNode { val: 30, left: None, right: None };
        let mut node_c = TreeNode { val: 40, left: None, right: None };
        let mut node_d = TreeNode { val: 50, left: None, right: None };
        let node_e = TreeNode { val: 60, left: None, right: None };

        //      a
        //       \
        //        b
        //       /
        //      c
        //       \
        //        d
        //         \
        //          e
        node_d.right = Some(Rc::new(RefCell::new(node_e)));
        node_c.right = Some(Rc::new(RefCell::new(node_d)));
        node_b.left = Some(Rc::new(RefCell::new(node_c)));
        node_a.right = Some(Rc::new(RefCell::new(node_b)));
        assert_eq!(tree_leaf_list(Rc::new(RefCell::new(node_a))), &[60]);
    }
    #[test]
    fn test_tree_leaf_list_recursive_03() {
        let mut node_a = TreeNode { val: 20, left: None, right: None };
        let mut node_b = TreeNode { val: 30, left: None, right: None };
        let mut node_c = TreeNode { val: 40, left: None, right: None };
        let mut node_d = TreeNode { val: 50, left: None, right: None };
        let node_e = TreeNode { val: 60, left: None, right: None };

        //      a
        //       \
        //        b
        //       /
        //      c
        //       \
        //        d
        //         \
        //          e
        node_d.right = Some(Rc::new(RefCell::new(node_e)));
        node_c.right = Some(Rc::new(RefCell::new(node_d)));
        node_b.left = Some(Rc::new(RefCell::new(node_c)));
        node_a.right = Some(Rc::new(RefCell::new(node_b)));
        assert_eq!(leaf_list_recursive(Rc::new(RefCell::new(node_a))), &[60]);
    }
}
