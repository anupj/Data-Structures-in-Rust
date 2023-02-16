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

/// This function takes in the root of a binary tree.
/// It should return an array containing all values
/// of the tree in breadth-first order
pub fn tree_includes(root: TreeNodeRef, target: i32) -> bool {
    // println!("The root node is {:?}", root);
    let mut queue: VecDeque<TreeNodeRef> = VecDeque::new();
    queue.push_back(root);
    while !queue.is_empty() {
        let current: Rc<RefCell<TreeNode>> = queue.pop_front().unwrap();

        if current.borrow().val == target {
            return true;
        }

        if let Some(left) = &current.borrow().left {
            queue.push_back(left.to_owned());
        };
        if let Some(right) = &current.borrow().right {
            queue.push_back(right.to_owned());
        };
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadth_first_values_00() {
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
        let node_d = TreeNode {
            val: 50,
            left: None,
            right: None,
        };
        let node_e = TreeNode {
            val: 60,
            left: None,
            right: None,
        };
        let node_f = TreeNode {
            val: 70,
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
        assert_eq!(tree_includes(Rc::new(RefCell::new(node_a)), 60), true);
    }

    #[test]
    fn test_breadth_first_values_01() {
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
        let node_d = TreeNode {
            val: 50,
            left: None,
            right: None,
        };
        let node_e = TreeNode {
            val: 60,
            left: None,
            right: None,
        };
        let node_f = TreeNode {
            val: 70,
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
        assert_eq!(tree_includes(Rc::new(RefCell::new(node_a)), 600), false);
    }

    #[test]
    fn test_breadth_first_values_02() {
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
        let node_d = TreeNode {
            val: 50,
            left: None,
            right: None,
        };
        let mut node_e = TreeNode {
            val: 60,
            left: None,
            right: None,
        };
        let mut node_f = TreeNode {
            val: 70,
            left: None,
            right: None,
        };
        let node_g = TreeNode {
            val: 80,
            left: None,
            right: None,
        };
        let node_h = TreeNode {
            val: 90,
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
        assert_eq!(tree_includes(Rc::new(RefCell::new(node_a)), 90), true);
    }

    #[test]
    fn test_breadth_first_values_03() {
        let node_a = TreeNode {
            val: 20,
            left: None,
            right: None,
        };
        assert_eq!(tree_includes(Rc::new(RefCell::new(node_a)), 20), true);
    }

    #[test]
    fn test_breadth_first_values_04() {
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

        let node_x = TreeNode {
            val: 70,
            left: None,
            right: None,
        };
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
        assert_eq!(tree_includes(Rc::new(RefCell::new(node_a)), 95), false);
    }
}
