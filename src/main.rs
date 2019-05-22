fn main() {
	let t = vec![1, 2, 11, 14, 20, 21, 24, 101, 111, 123, 1123, 12345, 1_234_567_891];
	for x in t {
		println!("{}: {}", x, Solution::number_to_words(x));
	}
}

struct Solution;

impl Solution {
	pub fn number_to_words(num: i32) -> String {

		if num == 0 {
			return String::from("Zero");
		}

		let digits = vec![
			"Zero", "One", "Two", "Three", "Four", "Five", "Six",
			"Seven", "Eight", "Nine", "Ten",
			"Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen",
			"Sixteen", "Seventeen", "Eighteen", "Nineteen",
		];
		let tens = vec![
			"Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
		];
		let convert = |n: usize| {
			let mut n = n;
			let mut ret = String::with_capacity(30);
			let x = n / 100;
			n %= 100;
			if x > 0 {
				ret.push_str(format!("{} Hundred ", digits[x]).as_str());
			}

			if n >= 20 {
				if n % 10 == 0 {
					ret.push_str(format!("{} ", tens[n / 10 - 2]).as_str());
				} else {
					ret.push_str(format!("{} {} ", tens[n / 10 - 2], digits[n % 10]).as_str());
				}
			} else if n > 0 {
				ret.push_str(format!("{} ", digits[n]).as_str());
			}

			ret
		};

		let units_str = vec![
			"Billion", "Million", "Thousand"
		];
		let units_val = vec![1000_000_000, 1000_000, 1000];
		let mut ret = String::with_capacity(120);
		let mut num = num;

		for (idx, unit) in units_val.into_iter().enumerate() {
			let x = num / unit;
			num %= unit;
			if x > 0 {
				ret.push_str(format!("{}{} ", convert(x as usize), units_str[idx]).as_str());
			}
		}

		if num > 0 {
			ret.push_str(format!("{}", convert(num as usize)).as_str());
		}

		ret.trim().to_string()
	}
}