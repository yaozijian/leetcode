fn main() {
	let t = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15, 16, 19, 26, 29, 100, 120, 124, 138, 1000, 1049, 2000, 3000, 3999];
	for x in t {
		println!("{}: {}", x, Solution::int_to_roman(x));
	}
}

struct Solution;

impl Solution {
	pub fn int_to_roman(num: i32) -> String {
		use std::collections::HashMap;
		let mut items = HashMap::with_capacity(4);
		items.insert(1, vec!['I', 'V', 'X']);
		items.insert(10, vec!['X', 'L', 'C']);
		items.insert(100, vec!['C', 'D', 'M']);
		items.insert(1000, vec!['M', 'M', 'M']);

		let mut num = num;
		let mut ret = String::with_capacity(20);
		let bits = vec![1000, 100, 10, 1];
		for bit in bits {
			let x = num / bit;
			num = num % bit;
			if x <= 0 {
				continue;
			}
			let cur = items.get(&bit).unwrap();
			match x {
				1 | 2 | 3 => for _ in 0..x { ret.push(cur[0]); }
				4 => {
					ret.push(cur[0]);
					ret.push(cur[1]);
				}
				5 | 6 | 7 | 8 => {
					ret.push(cur[1]);
					for _ in 0..x - 5 { ret.push(cur[0]); }
				}
				9 => {
					ret.push(cur[0]);
					ret.push(cur[2]);
				}
				_ => {}
			}
		}

		ret
	}
}