use list::List;

mod list;
mod leetcode_19;
mod leetcode_21;
mod leetcode_24;
mod leetcode_25;
mod leetcode_61;

struct Solution;

fn main() {
	leetcode_19();
	leetcode_21();
	leetcode_24();
	leetcode_25();
	leetcode_61();
}

fn leetcode_19() {
	println!("\nLeetCode 19");
	let x: List = vec![1, 2, 3, 4, 5].into_iter().into();
	x.show();
	let x = List(<Solution as leetcode_19::LeetCode19>::remove_nth_from_end(x.0, 2));
	x.show();
}

fn leetcode_21() {
	println!("\nLeetCode 21");
	let x: List = vec![1, 2, 8, 9, 10].into_iter().into();
	let y: List = vec![3, 4, 5, 6, 7].into_iter().into();
	x.show();
	y.show();
	let x = List(<Solution as leetcode_21::LeetCode21>::merge_two_lists(x.0,y.0));
	x.show();

	let x: List = vec![1, 2, 8, 9, 10].into_iter().into();
	let y: List = vec![3, 4, 5, 6, 7].into_iter().into();
	x.show();
	y.show();
	let x = List(<Solution as leetcode_21::LeetCode21>::merge_two_lists2(x.0,y.0));
	x.show();
}

fn leetcode_24() {
	println!("\nLeetCode 24");
	let x: List = vec![1, 2, 3, 4, 5].into_iter().into();
	x.show();
	let x = List(<Solution as leetcode_24::LeetCode24>::swap_pairs(x.0));
	x.show();
}

fn leetcode_25() {
	println!("\nLeetCode 25");
	let x: List = vec![1, 2, 3, 4, 5].into_iter().into();
	x.show();
	let x = List(<Solution as leetcode_25::LeetCode25>::reverse_k_group(x.0, 2));
	x.show();
}

fn leetcode_61() {
	println!("\nLeetCode 61");
	let x: List = vec![1, 2, 3, 4, 5].into_iter().into();
	x.show();
	let x = List(<Solution as leetcode_61::LeetCode61>::rotate_right(x.0, 2));
	x.show();
}