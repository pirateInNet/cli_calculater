use to_get_input::{get_str,get_two_numbers};

fn main() {
	let operator = get_str("Inter an operator(like + - / *)");
	match operator.trim() {
		"+" => {
			let numbers: [f64;2] = get_two_numbers();
			println!("{} + {} = {}", numbers[0], numbers[1], numbers[0]+numbers[1]);
		},
		"-" => {
			let numbers: [f64;2] = get_two_numbers();
			println!("{} - {} = {}", numbers[0], numbers[1], numbers[0]-numbers[1]);
		},
		"*" => {
			let numbers: [f64;2] = get_two_numbers();
			println!("{} * {} = {}", numbers[0], numbers[1], numbers[0]*numbers[1]);
		},
		"/" => {
			let numbers: [f64;2] = get_two_numbers();
			if numbers[1]==0.0 {println!("you can't devide by 0");};
			println!("{} / {} = {}", numbers[0], numbers[1], numbers[0]/numbers[1]);
		},
		_ => println!("you intered wrong!")
	}
}

mod to_get_input {
	use std::io::{stdin,stdout,Write};
	pub fn get_str(prompt: &str) -> String {
			let mut s = String::new();
			print!("{} :",prompt);
			let _=stdout().flush();
			stdin()
				.read_line(&mut s)
				.expect("Did not enter a correct string");
			s
	}
	pub fn get_two_numbers() -> [f64;2] {
		let mut s = String::new();
		println!("inter 2 numbers");
		print!("number 1: ");
		let _=stdout().flush();
		stdin()
			.read_line(&mut s)
			.expect("Did not enter a correct string");
		let num1: f64 = s.trim()
						.parse()
						.expect("We can't parse this string into float - num1");
		print!("number 2: ");
		let _=stdout().flush();
		s.clear();
		stdin()
			.read_line(&mut s)
			.expect("Did not enter a correct string");
		let num2: f64 = s.trim()
						.parse()
						.expect("We can't parse this string into float - num2");
		[num1,num2]
	}
}
