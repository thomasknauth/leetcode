use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Deref;
use std::rc::Rc;

// Note to self: I am still not sure why we need Rc<RefCell<...>> here. My understanding is that every node has a single incoming edge, i.e, no node actually has shared ownership.

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[derive(Debug)]
pub struct TreeNodeV2 {
    pub val: i32,
    pub left: Option<Box<TreeNodeV2>>,
    pub right: Option<Box<TreeNodeV2>>,
}

// For funsies, implement same functionality with a Option<Box<TreeNode>> for the left and right branches to see if this, in principle, also works.
impl TreeNodeV2 {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNodeV2 {
            val,
            left: None,
            right: None,
        }
    }

    pub fn right_side_view(root: Option<Box<Self>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        if let Some(root_rc) = root {
            let mut level_nodes = vec![&root_rc];

            while !level_nodes.is_empty() {
                let mut next_level_nodes = vec![];
                let mut last_node = 0;

                for node in level_nodes {
                    last_node = node.val;
                    if let Some(left_rc) = &node.left {
                        next_level_nodes.push(left_rc);
                    }
                    if let Some(right_rc) = &node.right {
                        next_level_nodes.push(right_rc);
                    }
                }
                result.push(last_node);
                level_nodes = next_level_nodes;
            }
        }
        result
    }

    // My puny attempt at an implementation. I looked at the much more idiomatic way spit out by an AI, but keep my version here anyhow. It's a learning exercise after all.
    pub fn from_str(s: &str) -> Option<Box<Self>> {
        let parsed = parse_list(s);

        if parsed.is_empty() {
            return None;
        }

        let mut root = if let Some(val) = parsed.first().unwrap() {
            Some(Box::new(Self::new(*val)))
        } else {
            None
        };

        let mut q = VecDeque::<&mut Option<Box<Self>>>::new();
        q.push_back(&mut root);

        for chunk in parsed[1..].chunks(2) {
            let (left, right) = match chunk {
                [left, right] => (*left, *right),
                [left] => (*left, None),
                _ => panic!(),
            };

            if let Some(node) = q.pop_front() {
                if let Some(n) = node {
                    n.left = if let Some(v) = left {
                        Some(Box::new(TreeNodeV2::new(v)))
                    } else {
                        None
                    };
                    if n.left.is_some() {
                        q.push_back(&mut n.left);
                    }
                    n.right = if let Some(v) = right {
                        Some(Box::new(TreeNodeV2::new(v)))
                    } else {
                        None
                    };
                    if n.right.is_some() {
                        q.push_back(&mut n.right);
                    }
                }
            }
        }
        root
    }
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_str(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
        let parsed = parse_list(s);

        let mut root = None;
        let mut q = VecDeque::<Rc<RefCell<TreeNode>>>::new();

        let mut do_left = true;

        for p in parsed {
            if root.is_none() {
                if p.is_none() {
                    return root;
                }

                let rc = Rc::new(RefCell::new(TreeNode::new(p.unwrap())));
                root = Some(rc.clone());
                q.push_back(rc.clone());
                q.push_back(rc);
            } else {
                let node_rc = q.pop_front().unwrap();

                if let Some(v) = p {
                    let new_rc = Rc::new(RefCell::new(TreeNode::new(v)));
                    q.push_back(new_rc.clone());
                    q.push_back(new_rc.clone());

                    match do_left {
                        true => node_rc.borrow_mut().left = Some(new_rc),
                        false => node_rc.borrow_mut().right = Some(new_rc),
                    }
                }

                do_left = !do_left;
            }
        }

        root
    }
}

fn parse_list(input: &str) -> Vec<Option<i32>> {
    input
        // Trim surrounding brackets
        .trim_matches(&['[', ']'][..])
        // Split by commas
        .split(',')
        // Map each piece
        .map(|s| {
            let s = s.trim(); // remove spaces
            if s == "null" || s.is_empty() {
                None
            } else {
                Some(s.parse::<i32>().unwrap())
            }
        })
        .collect()
}

struct Solution {}

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        if let Some(root_rc) = root {
            let mut level_nodes = vec![root_rc];

            while !level_nodes.is_empty() {
                let mut next_level_nodes = vec![];
                let mut last_node = 0;

                for node in level_nodes {
                    last_node = node.borrow().val;
                    if let Some(left_rc) = &node.borrow().left {
                        next_level_nodes.push(left_rc.clone());
                    }
                    if let Some(right_rc) = &node.borrow().right {
                        next_level_nodes.push(right_rc.clone());
                    }
                }
                result.push(last_node);
                level_nodes = next_level_nodes;
            }
        }
        result
    }
}

pub fn compare_trees(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Box<TreeNodeV2>>) -> bool {
    match (a, b) {
        (Some(ref node_a), Some(ref node_b)) => {
            node_a.borrow().val == node_b.val
                && compare_trees(&node_a.borrow().left, &node_b.left)
                && compare_trees(&node_a.borrow().right, &node_b.right)
        }
        (None, None) => true,
        _ => false,
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_side_view() {
        let test_cases = vec![
            ("[1,2,3,null,5,null,4]", vec![1, 3, 4]),
            ("[1,2,3,4,null,null,null,5]", vec![1, 3, 4, 5]),
            ("[1,null,3]", vec![1, 3]),
            ("[]", vec![]),
        ];

        for (input, expected_result) in test_cases {
            assert_eq!(
                Solution::right_side_view(TreeNode::from_str(input)),
                expected_result
            );
        }
    }

    #[test]
    fn test_from_str() {
        assert_eq!(TreeNode::from_str("[]"), None);
        assert_eq!(
            TreeNode::from_str("[1]"),
            Some(Rc::new(RefCell::new(TreeNode::new(1))))
        );
        assert_eq!(
            TreeNode::from_str("[1,null,2]"),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(2))))
            })))
        );
    }

    #[test]
    fn test_treenodev2_from_str() {
        let test_cases = vec![
            "[1,2,3,null,5,null,4]",
            "[1,2,3,4,null,null,null,5]",
            "[1,null,3]",
            "[]",
        ];
        for s in test_cases {
            println!("{}", s);
            assert!(compare_trees(
                &TreeNode::from_str(s),
                &TreeNodeV2::from_str(s)
            ));
        }
    }
}
