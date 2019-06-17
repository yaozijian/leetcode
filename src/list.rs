use std::iter::DoubleEndedIterator;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
	pub val: i32,
	pub next: Option<Box<ListNode>>
}

pub struct List(pub Option<Box<ListNode>>);

impl ListNode {
	#[inline]
	pub fn new(val: i32) -> Self {
		ListNode {
			next: None,
			val
		}
	}
}

impl List {
	pub fn show(&self) {
		let mut cur = &self.0;
		let mut node;
		while cur.is_some() {
			node = cur.as_ref().unwrap();
			print!("{} ->", node.val);
			cur = &node.next;
		}
		println!();
	}
}

impl<T> From<T> for List where T: DoubleEndedIterator<Item=i32> {
	fn from(v: T) -> Self {
		let add = |x, h| {
			let mut n = ListNode::new(x);
			n.next = h;
			List(Some(Box::new(n)))
		};

		let mut h = List(None);

		for x in v.rev() {
			h = add(x, h.0);
		}

		h
	}
}