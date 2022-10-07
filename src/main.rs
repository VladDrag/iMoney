mod models;


fn main() {
    let expense = models::Expense::new(String::from("food"), 1000.0);
	println!("expenses for {} are {}", expense.get_name(), expense.get_amount());

	let income = models::Income::new(String::from("salary"), 10000.0);
	println!("income for {} is {}", income.name(), income.amount());
}
