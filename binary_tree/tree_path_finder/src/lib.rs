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
/// and a target value.
/// It should return a `vec` representing a path to
/// the target value. if the target value is not found,
/// then return an empty `vec`
pub fn tree_path_finder(root: TreeNodeRef, target: i32) -> Option<Vec<i32>> {
    if let Some(mut result) = tree_path_finder_helper(Some(root), target) {
        result.reverse();
        return Some(result);
    }
    None
}

fn tree_path_finder_helper(
    root: Option<TreeNodeRef>,
    target: i32,
) -> Option<Vec<i32>> {
    if root.is_none() {
        return None;
    }
    // Get the value out of the `Option`
    let root = root.unwrap();

    // Store the root value
    let root_val: i32 = root.borrow().val;

    // If this is the target
    // return it in a `vec`
    if root_val == target {
        return Some(vec![root_val]);
    }

    // Follow the left path and
    // call the function recursively
    let mut left_path =
        tree_path_finder_helper(root.borrow().left.clone(), target);
    if !left_path.is_none() {
        left_path.as_mut().unwrap().push(root_val);
        return left_path;
    }

    // Follow the right path and
    // call the function recursively
    let mut right_path =
        tree_path_finder_helper(root.borrow().right.clone(), target);
    if !right_path.is_none() {
        right_path.as_mut().unwrap().push(root_val);
        return right_path;
    }

    None
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_tree_path_finder_values_00() {
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
            tree_path_finder(Rc::new(RefCell::new(node_a)), 60),
            Some(vec![20, 30, 60])
        );
    }

    #[test]
    fn test_tree_path_finder_values_01() {
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
            tree_path_finder(Rc::new(RefCell::new(node_a)), 70),
            Some(vec![20, 40, 70])
        );
    }

    #[test]
    fn test_tree_path_finder_values_02() {
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
            tree_path_finder(Rc::new(RefCell::new(node_a)), 90),
            Some(vec![20, 40, 70, 90])
        );
    }

    #[test]
    fn test_tree_path_finder_values_03() {
        let node_a = TreeNode { val: 20, left: None, right: None };
        assert_eq!(
            tree_path_finder(Rc::new(RefCell::new(node_a)), 20),
            Some(vec![20])
        );
    }

    #[test]
    fn test_tree_path_finder_values_04() {
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
            tree_path_finder(Rc::new(RefCell::new(node_a.clone())), 60),
            Some(vec![20, 30, 40, 50, 60])
        );
        assert_eq!(tree_path_finder(Rc::new(RefCell::new(node_a)), 95), None);
    }
}
