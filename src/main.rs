use std::cell::RefCell;
use std::rc::Rc;

fn main() {
	let mut head = TreeNode::new(1);
	let mut left = TreeNode::new(2);
	head.left = Solution::item_from_node(left);
	/*
	left.left = Solution::item_from_val(3);
	left.right = Solution::item_from_val(4);
	head.left = Solution::item_from_node(left);

	let mut r = TreeNode::new(5);
	r.right = Solution::item_from_val(6);
	head.right = Solution::item_from_node(r);
	*/

	let mut head = Solution::item_from_node(head);
	Solution::traverse(&head);
	println!("\n\n");
	Solution::flatten(&mut head);
	Solution::traverse(&head);
	println!("\n\n");
}

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

struct Solution;

impl Solution {
	pub fn item_from_node(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
		Some(Rc::new(RefCell::new(n)))
	}
	pub fn traverse(root: &Option<Rc<RefCell<TreeNode>>>) {
		if root.is_none() {
			print!(" null ");
		} else {
			let x = root.as_ref().unwrap().as_ref();
			print!(" {} ", x.borrow().val);
			Solution::traverse(&x.borrow().left);
			Solution::traverse(&x.borrow().right);
		}
	}
	pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
		expand(root);

		fn expand(tree: &mut Option<Rc<RefCell<TreeNode>>>) {
			if tree.is_none() { return; }

			let root = tree.as_ref().unwrap().clone();
			let root = root.as_ref();
			let mut left = root.borrow_mut().left.take();
			let mut right = root.borrow_mut().right.take();

			expand(&mut right);
			expand(&mut left);

			if left.is_some() && right.is_some() {
				let mut rcnode = left.as_ref().unwrap().clone();
				let mut refnode = rcnode.as_ref();
				let mut c = refnode.borrow().right.is_some();
				while c {
					rcnode = rcnode.clone().as_ref().borrow().right.as_ref().unwrap().clone();
					refnode = rcnode.as_ref();
					c = refnode.borrow().right.is_some();
				}
				refnode.borrow_mut().right.replace(right.unwrap());
				root.borrow_mut().right.replace(left.unwrap());
			} else if right.is_some() {
				root.borrow_mut().right.replace(right.unwrap());
			} else if left.is_some() {
				root.borrow_mut().right.replace(left.unwrap());
			}
		}
	}
}