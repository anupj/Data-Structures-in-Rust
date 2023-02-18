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
    val: i32,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

/// This function takes in the root of a binary tree
/// that contains number values.
/// It should return the maximum sum of any root to
/// leaf path within the tree.
/// We are going for the recursive approach here
pub fn max_path_sum(root: Option<TreeNodeRef>) -> i32 {
    // if the node is pointing to
    // `None` return a minimum value
    if root.is_none() {
        return i32::MIN;
    }
    let root = root.as_ref().unwrap();

    // base case
    // if this is a leaf node then return the val
    if root.borrow().left.is_none() && root.borrow().right.is_none() {
        return root.borrow().val;
    }

    let max_child_path_sum = cmp::max(
        max_path_sum(root.borrow().left.clone()),
        max_path_sum(root.borrow().right.clone()),
    );

    root.borrow().val + max_child_path_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_path_sum_values_00() {
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
        assert_eq!(max_path_sum(Some(Rc::new(RefCell::new(node_a)))), 130);
    }

    #[test]
    fn test_max_path_sum_values_01() {
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
        assert_eq!(max_path_sum(Some(Rc::new(RefCell::new(node_a)))), 130);
    }

    #[test]
    fn test_max_path_sum_values_02() {
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
        assert_eq!(max_path_sum(Some(Rc::new(RefCell::new(node_a)))), 220);
    }

    #[test]
    fn test_max_path_sum_values_03() {
        let node_a = TreeNode { val: 20, left: None, right: None };
        assert_eq!(max_path_sum(Some(Rc::new(RefCell::new(node_a)))), 20);
    }

    #[test]
    fn test_max_path_sum_values_04() {
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
        assert_eq!(max_path_sum(Some(Rc::new(RefCell::new(node_a)))), 200);
    }
}
