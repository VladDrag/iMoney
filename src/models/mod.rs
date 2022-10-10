pub struct Expense {
	name : String,
	amount : f64,
}

impl Expense {
	pub fn new(name: String, amount: f64) -> Expense {
		Expense {
			name: name,
			amount: amount
		}
	}
	pub fn get_amount(&self) -> f64 { self.amount}
	pub fn get_name(&self) -> &String { &self.name }
	pub fn set_amount(&mut self, amount: f64) { self.amount = amount }
	pub fn set_name(&mut self, name: String) { self.name = name }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////


pub struct Income {
	name: String,
	amount: f64,
}


impl Income {
	pub fn new(name: String, amount: f64) -> Income {
		Income {
			name: name,
			amount: amount
		}
	}
	pub fn get_amount(&self) -> f64 { self.amount }
	pub fn get_name(&self) -> &String { &self.name }
	pub fn set_amount(&mut self, amount: f64) -> { self.amount = amount }
	pub fn set_name(&mut self, name: String) -> { self.name = name }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Member {
	name: String,
	income: Vec<Income>,
	expenses: Vec<Expense>
}

impl Member {
	pub fn new(name: String) -> Member {
		Member {
			name: name,
			income: Vec::new(),
			expenses: Vec::new()
		}
	}
	pub fn get_name(&self) -> &String { &self.name }
	pub fn get_income(&self) -> &Vec<Income> { &self.income }
	pub fn get_expenses(&self) -> &Vec<Expense> { &self.expenses }
	pub fn set_name(&mut self, name: String) { self.name = name }
	pub fn set_income(&mut self, income: Vec<Income>) { self.income = income }
	pub fn set_expenses(&mut self, expenses: Vec<Expense>) { self.expenses = expenses }
	pub fn add_income(&mut self, income: Income) { self.income.push(income) }
	pub fn add_expense(&mut self, expense: Expense) { self.expenses.push(expense) }
	pub fn total_income (&self) -> i64 { 
		let mut total = 0;
		for income in &self.income {
			total += income.get_amount();
		}
		total
	}
	pub fn total_expenses (&self) -> i64 {
		let mut total = 0;
		for expense in &self.expenses {
			total += expense.get_amount();
		}
		total
	}
}