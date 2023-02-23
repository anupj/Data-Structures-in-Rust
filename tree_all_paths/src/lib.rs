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
/// It should return a 2-dimensional array where
/// each subarray represents a root-to-leaf path
/// in the tree.
/// The order within an individual path must start
/// at the root and end at the leaf, but the relative
/// order among paths in the outer array does not
/// matter.
/// You may assume that the input tree is non-empty.
///
/// If the input is
///        -1
///      /    \
///    -6     -5
///   /  \      \
/// -3   -4    -13
///
/// The result is:  
/// [
///   [ -1, -6, -3],
///   [-1, -6, -4],
///   [-1, -5, -13]
/// ]
///
/// Time: O(n)
/// Space: O(n)
/// This will require a recursive approach.
pub fn tree_all_paths(root: Option<&TreeNodeRef>) -> Vec<Vec<i32>> {
    // check if `root` has `Some`thing in it
    if let Some(root) = root {
        // Check if this is a leaf node
        if root.borrow().left.is_none() && root.borrow().right.is_none() {
            // if leaf, then return [[root.val]]
            return vec![vec![root.borrow().val]];
        }

        let mut paths: Vec<Vec<i32>> = Vec::new();

        let left_sub_paths = tree_all_paths(root.borrow().left.as_ref());
        for mut subpath in left_sub_paths {
            subpath.insert(0, root.borrow().val);
            paths.push(subpath);
        }
        let right_sub_paths = tree_all_paths(root.borrow().right.as_ref());
        for mut subpath in right_sub_paths {
            subpath.insert(0, root.borrow().val);
            paths.push(subpath);
        }

        paths
    } else {
        // return an empty `Vec<Vec<i32>>`
        // NOTE: the return type is inferred
        // by the compiler
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_all_paths_00() {
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
            tree_all_paths(Some(&Rc::new(RefCell::new(node_a)))),
            vec![vec![20, 30, 50], vec![20, 30, 60], vec![20, 40, 70],]
        );
    }

    #[test]
    fn test_tree_all_paths_01() {
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
            tree_all_paths(Some(&Rc::new(RefCell::new(node_a)))),
            vec![vec![20, 30, 50], vec![20, 30, 60, 80], vec![20, 40, 70, 90],]
        );
    }

    #[test]
    fn test_tree_all_paths_02() {
        let node_a = TreeNode { val: 20, left: None, right: None };
        assert_eq!(
            tree_all_paths(Some(&Rc::new(RefCell::new(node_a)))),
            vec![vec![20]]
        );
    }

    #[test]
    fn test_tree_all_paths_03() {
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
            tree_all_paths(Some(&Rc::new(RefCell::new(node_a)))),
            vec![vec![20, 30, 40, 70], vec![20, 30, 40, 50, 60],]
        );
    }

    #[test]
    fn test_tree_all_paths_04() {
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
        assert_eq!(
            tree_all_paths(Some(&Rc::new(RefCell::new(node_a)))),
            vec![
                vec![20, 30, 50],
                vec![20, 30, 60, 80],
                vec![20, 30, 60, 90],
                vec![20, 40, 70, 95],
            ]
        );
    }
}
