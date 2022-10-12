#![allow(dead_code)]
#![allow(non_snake_case)]

mod models;
use models::member::Member;
mod collect_income;
mod collect_expenses;



fn main() {
	let mut vlad = Member::new(String::from("Vlad"));
	collect_expenses::get_expenses(&mut vlad);

	println!("Expenses for {} are :", vlad.get_name());
	println!("~~~~~~~~~~~~~~~~~~~~");
	for i in 0..vlad.get_expenses().len() - 1 {
		println!("{}: {}", vlad.get_expenses()[i].get_name(), vlad.get_expenses()[i].get_amount());
	}

	collect_income::get_income(&mut vlad);
	println!("Income for {} are :", vlad.get_name());
	println!("~~~~~~~~~~~~~~~~~~~~");
	for i in 0..vlad.get_income().len() - 1 {
		println!("{}: {}", vlad.get_income()[i].get_name(), vlad.get_income()[i].get_amount());
	}
}
