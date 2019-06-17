use super::list::ListNode;
use super::Solution;

pub trait LeetCode19 {
	fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>>;
}

impl LeetCode19 for Solution {
	fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {

		// 1 加一个"哑节点"
		let dummy = Some(Box::new(ListNode { val: 0, next: head }));

		// 2 前进n个位置
		let mut f = &dummy.as_ref().unwrap().next;
		let mut idx = 0;

		while idx < n && f.is_some() {
			idx += 1;
			f = &f.as_ref().unwrap().next;
		}

		// 节点数量 < n ?
		if idx < n {
			return dummy.unwrap().next;
		}

		// 3 双指针法: s 在 f 后面 n 个位置
		let mut s = &dummy;
		while f.is_some() {
			f = &f.as_ref().unwrap().next;
			s = &s.as_ref().unwrap().next;
		}

		// 4 执行删除
		unsafe {
			let x = &s.as_ref().unwrap().next;
			let y = &x.as_ref().unwrap().next;
			let x = x as *const Option<Box<ListNode>>;
			let x = x as *mut Option<Box<ListNode>>;
			*x = y.clone();
		}

		dummy.unwrap().next
	}
}