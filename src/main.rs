#![allow(dead_code)]
#![allow(non_snake_case)]

mod models;
use models::member::Member;
mod collect_income;
mod collect_expenses;
use colored::Colorize;
use std::io;



fn main() {
	println!("{}", Colorize::green("Please enter your user name:"));

	let mut name = String::new();
	io::stdin().read_line(&mut name).unwrap();

	let mut member = Member::new(name);

	collect_income::get_income(&mut member);
	collect_expenses::get_expenses(&mut member);


	println!("Income and expenses for {} are :", Colorize::blue(member.get_name().as_str()));

	println!("{}", Colorize::blue("~~~~~~~~~~~~~~~~~~~~"));
	for i in 0..member.get_expenses().len() - 1 {
		let expense = member.get_expenses()[i].get_amount();
		let expense_name = member.get_expenses()[i].get_name().as_str();
		println!("{}: {}", 
						Colorize::green(expense_name), 
						expense);
	}

	println!("{}", Colorize::blue("~~~~~~~~~~~~~~~~~~~~"));
	for i in 0..member.get_income().len() - 1 {
		let income = member.get_income()[i].get_amount();
		let income_name = member.get_income()[i].get_name().as_str();

		println!("{}: {}", 
						Colorize::green(income_name), 
						income);
	}
}
