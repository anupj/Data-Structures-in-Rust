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

/// This function takes in the root of a binary tree.
/// It should return an array containing all values
/// of the tree in depth-first order
pub fn depth_first_values(root: TreeNodeRef) -> Vec<i32> {
    // println!("The root node is {:?}", root);
    let mut result = Vec::new();
    let mut stack = vec![root];
    while stack.len() > 0 {
        let current: Rc<RefCell<TreeNode>> = stack.pop().unwrap();
        result.push(current.borrow().val);

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
            depth_first_values(Rc::new(RefCell::new(node_a))),
            &[20, 30, 50, 60, 40, 70]
        );
    }

    #[test]
    fn test_depth_first_values_01() {
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
            depth_first_values(Rc::new(RefCell::new(node_a))),
            &[20, 30, 50, 60, 80, 40, 70]
        );
    }
}

