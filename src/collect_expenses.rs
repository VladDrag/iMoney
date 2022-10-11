use std::io;
use crate::models::Member;
use crate::models::Expense;

pub fn get_expenses(mem: &mut Member) {
	loop {
		println!("Enter expense name or 'q' to quit");

		let mut name = String::new();
		io::stdin().read_line(&mut name).expect("Failed to read line");
		if name.trim() == "q" {
			break;
		}
		// input.set_name(name.trim().to_string());

		println!("Enter expense amount");

		let mut amount = String::new();
		io::stdin().read_line(&mut amount).expect("Failed to read line");
		// input.set_amount(amount.trim().parse::<f32>().unwrap());
		let input = Expense::new(name.trim().to_string(), amount.trim().parse::<f32>().unwrap());
		mem.add_expense(input);
	}
}