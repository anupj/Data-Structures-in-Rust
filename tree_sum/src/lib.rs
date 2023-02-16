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
/// that contains number values. The function should
/// return the toal sum of all values in the tree.
pub fn tree_sum(root: TreeNodeRef) -> i32 {
    // println!("The root node is {:?}", root);
    let mut result = 0i32;
    let mut stack = vec![root];
    while !stack.is_empty() {
        let current: Rc<RefCell<TreeNode>> = stack.pop().unwrap();
        result += current.borrow().val;

        if let Some(right) = &current.borrow().right {
            stack.push(right.to_owned());
        };

        if let Some(left) = &current.borrow().left {
            stack.push(left.to_owned());
        };
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_first_values_00() {
        let mut node_a = TreeNode {
            val: 3,
            left: None,
            right: None,
        };
        let mut node_b = TreeNode {
            val: 11,
            left: None,
            right: None,
        };
        let mut node_c = TreeNode {
            val: 4,
            left: None,
            right: None,
        };
        let node_d = TreeNode {
            val: 4,
            left: None,
            right: None,
        };
        let node_e = TreeNode {
            val: -2,
            left: None,
            right: None,
        };
        let node_f = TreeNode {
            val: 1,
            left: None,
            right: None,
        };

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
        assert_eq!(tree_sum(Rc::new(RefCell::new(node_a))), 21);
    }

    #[test]
    fn test_depth_first_values_01() {
        let mut node_a = TreeNode {
            val: 1,
            left: None,
            right: None,
        };
        let mut node_b = TreeNode {
            val: 6,
            left: None,
            right: None,
        };
        let mut node_c = TreeNode {
            val: 0,
            left: None,
            right: None,
        };
        let node_d = TreeNode {
            val: 3,
            left: None,
            right: None,
        };
        let mut node_e = TreeNode {
            val: -6,
            left: None,
            right: None,
        };
        let mut node_f = TreeNode {
            val: 2,
            left: None,
            right: None,
        };
        let node_g = TreeNode {
            val: 2,
            left: None,
            right: None,
        };
        let node_h = TreeNode {
            val: 2,
            left: None,
            right: None,
        };

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
        assert_eq!(tree_sum(Rc::new(RefCell::new(node_a))), 10);
    }

    #[test]
    fn test_depth_first_values_02() {
        let node_a = TreeNode {
            val: 20,
            left: None,
            right: None,
        };
        assert_eq!(tree_sum(Rc::new(RefCell::new(node_a))), 20);
    }

    #[test]
    fn test_depth_first_values_03() {
        let mut node_a = TreeNode {
            val: 20,
            left: None,
            right: None,
        };
        let mut node_b = TreeNode {
            val: 30,
            left: None,
            right: None,
        };
        let mut node_c = TreeNode {
            val: 40,
            left: None,
            right: None,
        };
        let mut node_d = TreeNode {
            val: 50,
            left: None,
            right: None,
        };
        let node_e = TreeNode {
            val: 60,
            left: None,
            right: None,
        };

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
        assert_eq!(tree_sum(Rc::new(RefCell::new(node_a))), 200);
    }
}
