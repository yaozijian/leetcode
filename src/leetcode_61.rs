use super::list::ListNode;
use super::Solution;

pub trait LeetCode61 {
	fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>>;
}

impl LeetCode61 for Solution {
	fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
		// 1 Get List Length
		let mut head = head;
		let mut len = 0;
		let mut cur = &head;
		while cur.is_some() {
			len += 1;
			cur = &cur.as_ref().unwrap().next;
		}

		if len <= 1 {
			return head;
		}

		let k = k % len;
		if k <= 0 {
			return head;
		}

		//2 Get new head node
		let mut rest = len;
		let mut cur = &mut head;

		while cur.is_some() && rest != k {
			rest -= 1;
			cur = &mut cur.as_mut().unwrap().next;
		}

		let mut newhead = cur.take();
		// 3 New tail -> old head
		let mut cur = &mut newhead;
		while cur.is_some() {
			cur = &mut cur.as_mut().unwrap().next;
		}

		*cur = head.take();

		newhead
	}
}