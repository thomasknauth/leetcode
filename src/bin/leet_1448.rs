// Constraints
//
// The number of nodes in the binary tree is in the range [1, 10^5].
// Each node's value is between [-10^4, 10^4].

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

struct Solution {}

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

        if let None = root {
            return 0
        }

        let root = root.unwrap();

        let mut n = 0;
        let mut work = vec![(root.clone(), root.borrow().val)];
        while !work.is_empty() {
            let mut next = Vec::<(Rc<RefCell<TreeNode>>, i32)>::new();

            for (node_rc, max_val) in work {

                let node = node_rc.borrow();

                if node.val >= max_val {
                    n += 1;
                }

                if let Some(left_rc) = &node.left {
                    next.push((left_rc.clone(), i32::max(node.val, max_val)));
                }

                if let Some(right) = &node.right {
                    next.push((right.clone(), i32::max(node.val, max_val)));
                }
            }
            work = next;
        }
        n
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn gen_perfect_binary_tree() -> Rc<RefCell<TreeNode>> {

        let mut q = VecDeque::<(Rc<RefCell<TreeNode>>, i32)>::new();
        let root = Rc::new(RefCell::new(TreeNode::new(0)));
        q.push_back((root.clone(), 0));

        while let Some((node_rc, depth)) = q.pop_front() {
            let mut node = node_rc.borrow_mut();
            node.left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
            node.right = Some(Rc::new(RefCell::new(TreeNode::new(0))));

            if depth >= 11 {
                continue;
            }

            if let Some(rc) = &node.left {
                q.push_back((rc.clone(), depth+1));
            }
            if let Some(rc) = &node.right {
                q.push_back((rc.clone(), depth+1));
            }
        }
        root
    }

    #[test]
    fn it_works() {
        let root = Rc::new(RefCell::new(TreeNode::new(3)));

        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));

        if let Some(left_rc) = &root.borrow().left {
            left_rc.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        }

        if let Some(right_rc) = &root.borrow().right {
            right_rc.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
            right_rc.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        }

        assert_eq!(Solution::good_nodes(Some(root)), 4);

        let tree = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(Solution::good_nodes(tree), 1);

        let tree = Rc::new(RefCell::new(TreeNode::new(3)));
        tree.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        if let Some(left_rc) = &tree.borrow().left {
            left_rc.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
            left_rc.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        }
        assert_eq!(Solution::good_nodes(Some(tree)), 3);
        assert_eq!(Solution::good_nodes(Some(gen_perfect_binary_tree())), 8191);
    }
}
