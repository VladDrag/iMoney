mod models;
mod collect_income;
mod collect_expenses;

fn main() {
    // let expense = models::Expense::new(String::from("food"), 1000.0);
	// println!("expenses for {} are {}", expense.get_name(), expense.get_amount());

	// let income = models::Income::new(String::from("salary"), 10000.0);
	// println!("income for {} is {}", income.name(), income.amount());
	// let mut members: Vec<models::Member> = Vec::new();
	let mut vlad = models::Member::new(String::from("Vlad"));
	let mut lidia = models::Member::new(String::from("Lidia"));
	collect_expenses::get_expenses(&mut vlad);
	println!("First expnse for Vlad is: {}, amounting {}",
		vlad.get_expenses()[0].get_name(), vlad.get_expenses()[0].get_amount());
	
	println!("Second expnse for Vlad is: {}, amounting {}",
		vlad.get_expenses()[1].get_name(), vlad.get_expenses()[1].get_amount());
	// collect_expenses::get_expenses(&mut lidia);
}
