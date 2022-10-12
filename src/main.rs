mod models;
use models::Member::Member;
mod collect_income;
mod collect_expenses;

fn main() {
	let mut vlad = Member::new(String::from("Vlad"));
	// let mut lidia = Member::new(String::from("Lidia"));
	collect_expenses::get_expenses(&mut vlad);
	collect_income::get_income(&mut vlad);
}
