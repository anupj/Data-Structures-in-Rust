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
/// It should return the right-most value in the
/// bottom-most level of the tree.
/// _Assume that the input tree is non-empty_
/// If the input is
///        -1
///      /    \
///    -6     -5
///   /  \      \
/// -3   -4    -13
///     / \    /    
///    -2  6  7
/// Then the right-most value in the bottom-most
/// level is 7
/// Time: O(n)
/// Space: O(n)
pub fn bottom_right_value(root: TreeNodeRef) -> i32 {
    // `Rc.clone()` is cheap so use
    // liberally to make the borrow checker
    // happy
    let mut current = root.clone();
    let mut queue: VecDeque<TreeNodeRef> = VecDeque::new();
    queue.push_back(root);
    while !queue.is_empty() {
        current = queue.pop_front().unwrap();

        if let Some(left) = &current.borrow().left {
            queue.push_back(left.clone());
        };
        if let Some(right) = &current.borrow().right {
            queue.push_back(right.clone());
        };
    }
    let result = current.borrow().val;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bottom_right_value_00() {
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
        // should be `f` node
        assert_eq!(bottom_right_value(Rc::new(RefCell::new(node_a))), 70);
    }

    #[test]
    fn test_bottom_right_value_01() {
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
        // should be `h` node
        assert_eq!(bottom_right_value(Rc::new(RefCell::new(node_a))), 90);
    }

    #[test]
    fn test_bottom_right_value_02() {
        let node_a = TreeNode { val: 20, left: None, right: None };
        // should be `a` node
        assert_eq!(bottom_right_value(Rc::new(RefCell::new(node_a))), 20);
    }

    #[test]
    fn test_bottom_right_value_03() {
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
        // should be `e` node
        assert_eq!(bottom_right_value(Rc::new(RefCell::new(node_a))), 60);
    }

    #[test]
    fn test_bottom_right_value_04() {
        let mut node_a = TreeNode { val: 20, left: None, right: None };
        let mut node_b = TreeNode { val: 30, left: None, right: None };
        let mut node_c = TreeNode { val: 40, left: None, right: None };
        let node_d = TreeNode { val: 50, left: None, right: None };
        let mut node_e = TreeNode { val: 60, left: None, right: None };
        let mut node_f = TreeNode { val: 70, left: None, right: None };
        let node_g = TreeNode { val: 80, left: None, right: None };
        let node_h = TreeNode { val: 90, left: None, right: None };
        let node_i = TreeNode { val: 95, left: None, right: None };
        //          a
        //        /   \
        //       b     c
        //      / \     \
        //     d   e     f
        //        / \   /
        //       g   h  i
        node_e.right = Some(Rc::new(RefCell::new(node_h)));
        node_e.left = Some(Rc::new(RefCell::new(node_g)));
        node_f.left = Some(Rc::new(RefCell::new(node_i)));
        node_b.left = Some(Rc::new(RefCell::new(node_d)));
        node_b.right = Some(Rc::new(RefCell::new(node_e)));
        node_c.right = Some(Rc::new(RefCell::new(node_f)));
        node_a.left = Some(Rc::new(RefCell::new(node_b)));
        node_a.right = Some(Rc::new(RefCell::new(node_c)));
        // should be `i` node
        assert_eq!(bottom_right_value(Rc::new(RefCell::new(node_a))), 95);
    }
}
