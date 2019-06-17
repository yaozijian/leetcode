use super::list::ListNode;
use super::Solution;

pub trait LeetCode25 {
	fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>>;
}

impl LeetCode25 for Solution {
	fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
		if head.is_none() {
			return None;
		}

		let k = k as usize;
		let mut v = vec![];

		let mut next = head;
		let mut n;

		while next.is_some() {
			n = next.as_mut().unwrap().next.take();
			v.push(next);
			if v.len() == k {
				v[0].as_mut().unwrap().next = Solution::reverse_k_group(n, k as i32);
				for i in 1..k {
					v[i].as_mut().unwrap().next = v[i - 1].take();
				}
				return v[k - 1].take();
			} else {
				next = n;
			}
		}

		let cnt = v.len();
		if cnt > 1 {
			for i in (0..cnt - 1).rev() {
				v[i].as_mut().unwrap().next = v[i + 1].take();
			}
		}

		v[0].take()
	}
}