use list::List;

mod list;
mod leetcode_25;

struct Solution;

fn main() {
	leetcode_25();
}

fn leetcode_25(){
	let x: List = vec![1, 2, 3, 4, 5].into_iter().into();
	x.show();
	let x = List(<Solution as leetcode_25::LeetCode25>::reverse_k_group(x.0,2));
	x.show();
}
