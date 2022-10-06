mod models;


fn main() {
    let expense = models::Expense::new(String::from("food"), 1000.0);
	println!("expenses for {} are {}", expense.get_name(), expense.get_amount());
}
