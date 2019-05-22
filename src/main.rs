fn main() {
	assert_eq!(1, Solution::roman_to_int("I".to_string()));
	assert_eq!(2, Solution::roman_to_int("II".to_string()));
	assert_eq!(3, Solution::roman_to_int("III".to_string()));
	assert_eq!(4, Solution::roman_to_int("IV".to_string()));
	assert_eq!(5, Solution::roman_to_int("V".to_string()));
	assert_eq!(6, Solution::roman_to_int("VI".to_string()));
	assert_eq!(7, Solution::roman_to_int("VII".to_string()));
	assert_eq!(8, Solution::roman_to_int("VIII".to_string()));
	assert_eq!(9, Solution::roman_to_int("IX".to_string()));
	assert_eq!(10, Solution::roman_to_int("X".to_string()));

	assert_eq!(11, Solution::roman_to_int("XI".to_string()));
	assert_eq!(12, Solution::roman_to_int("XII".to_string()));
	assert_eq!(13, Solution::roman_to_int("XIII".to_string()));
	assert_eq!(14, Solution::roman_to_int("XIV".to_string()));
	assert_eq!(15, Solution::roman_to_int("XV".to_string()));
	assert_eq!(16, Solution::roman_to_int("XVI".to_string()));
	assert_eq!(17, Solution::roman_to_int("XVII".to_string()));
	assert_eq!(18, Solution::roman_to_int("XVIII".to_string()));
	assert_eq!(19, Solution::roman_to_int("XIX".to_string()));
	assert_eq!(20, Solution::roman_to_int("XX".to_string()));
	assert_eq!(30, Solution::roman_to_int("XXX".to_string()));
	assert_eq!(40, Solution::roman_to_int("XL".to_string()));
	assert_eq!(50, Solution::roman_to_int("L".to_string()));
	assert_eq!(60, Solution::roman_to_int("LX".to_string()));

	assert_eq!(100, Solution::roman_to_int("C".to_string()));
	assert_eq!(101, Solution::roman_to_int("CI".to_string()));
	assert_eq!(104, Solution::roman_to_int("CIV".to_string()));
	assert_eq!(105, Solution::roman_to_int("CV".to_string()));
	assert_eq!(108, Solution::roman_to_int("CVIII".to_string()));
	assert_eq!(109, Solution::roman_to_int("CIX".to_string()));
	assert_eq!(111, Solution::roman_to_int("CXI".to_string()));
	assert_eq!(120, Solution::roman_to_int("CXX".to_string()));

	assert_eq!(1000, Solution::roman_to_int("M".to_string()));
	assert_eq!(2000, Solution::roman_to_int("MM".to_string()));
	assert_eq!(3000, Solution::roman_to_int("MMM".to_string()));

	assert_eq!(2918, Solution::roman_to_int("MMCMXVIII".to_string()));
}

struct Solution;

impl Solution {
	pub fn roman_to_int(s: String) -> i32 {
		use std::collections::HashMap;
		let mut items = HashMap::with_capacity(10);
		items.insert('I', (vec!['I', 'V', 'X'], 1));
		items.insert('V', (vec!['I', 'V', 'X'], 1));
		items.insert('X', (vec!['X', 'L', 'C'], 10));
		items.insert('L', (vec!['X', 'L', 'C'], 10));
		items.insert('C', (vec!['C', 'D', 'M'], 100));
		items.insert('D', (vec!['C', 'D', 'M'], 100));
		items.insert('M', (vec!['M', 'M', 'M'], 1000));

		const ZERO: char = 0 as char;
		//// 0 first char 1 count 2 sub
		let mut status = (ZERO, 0, 0);
		let mut cur = items.get(&'I').unwrap();

		let add = |status: &mut (char, i32, i32), cur: &(Vec<char>, i32), ch: char| {
			let mut ret = false;
			let list = &cur.0;
			let unit = cur.1;
			if ch == list[0] { status.1 += 1; }
			else if ch == list[1] { status.2 = 4 * unit; }
			else if ch == list[2] { status.2 = 9 * unit; }
			else if status.0 == list[0] {
				status.2 = status.1 * unit;
				ret = true;
			} else {
				status.2 = (4 + status.1) * unit;
				ret = true;
			}
			ret
		};

		let mut sum = 0;
		for ch in s.chars() {
			if status.0 == ZERO {
				status = (ch, 1, 0);
				cur = &items[&ch];
			} else {
				let r = add(&mut status, &cur, ch);
				if status.2 != 0 {
					sum += status.2;
					if r {
						status = (ch, 1, 0);
						cur = &items[&ch];
					}else{
						status = (ZERO, 0, 0);
					}
				}
			}
		}

		if status.0 != ZERO {
			add(&mut status, &cur, ZERO);
			sum += status.2;
		}

		sum
	}
}