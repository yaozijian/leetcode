use super::list::ListNode;
use super::Solution;

pub trait LeetCode21 {
	fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
	fn merge_two_lists2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

impl LeetCode21 for Solution {
	fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		if l1.is_none() {
			return l2;
		} else if l2.is_none() {
			return l1;
		}

		let mut head_node = l1;
		let mut insert_before = &mut head_node;

		// 因为要直接将l2表中的节点,插入到l1表中,所以这里不使用引用类型
		// 而直接使用所有权类型
		let mut next_node = l2;
		let mut cur_node;

		while next_node.is_some() {

			// 取出l2的头结点和下一个结点
			let tmp_node = next_node.as_mut().unwrap().next.take();
			cur_node = next_node;
			next_node = tmp_node;

			let cur_val = cur_node.as_ref().unwrap().val;

			// 找到头结点的插入位置
			while insert_before.is_some() && cur_val > insert_before.as_ref().unwrap().val {
				insert_before = &mut insert_before.as_mut().unwrap().next;
			}

			// 要插入节点的下一个结点,是在其之前插入的节点。注意：使用take()获取Option<T>中有所有权的T。
			cur_node.as_mut().unwrap().next = insert_before.take();
			*insert_before = cur_node;
			insert_before = &mut insert_before.as_mut().unwrap().next;
		}

		head_node
	}

	fn merge_two_lists2(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		if l1.is_none() {
			return l2;
		} else if l2.is_none() {
			return l1;
		}

		let n1 = l1.as_mut().unwrap();
		let n2 = l2.as_mut().unwrap();
		if n1.val < n2.val {
			n1.next = Solution::merge_two_lists2(n1.next.take(), l2);
			l1
		} else {
			n2.next = Solution::merge_two_lists2(std::mem::replace(&mut n1.next, None), l1);
			l2
		}
	}
}