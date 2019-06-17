use super::list::ListNode;
use super::Solution;

pub trait LeetCode24 {
	fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

impl LeetCode24 for Solution {
	fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		if head.is_none() {
			return None;
		}

		let mut head = head;
		let h = &mut head;
		// 取出分离的第2个节点
		let mut s = h.as_mut().unwrap().next.take();
		// 没有第2个节点,直接返回
		if s.is_none() {
			return head;
		}

		// 取第3个节点
		let t = s.as_mut().unwrap().next.take();
		// 第1个节点变成第2个节点;递归处理剩余部分
		h.as_mut().unwrap().next = Solution::swap_pairs(t);
		// 第2个节点链接到第一个节点
		s.as_mut().unwrap().next = head;
		// 第2个节点成为新的头节点
		s
	}
}