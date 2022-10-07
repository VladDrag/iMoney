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
	pub fn amount(&self) -> f64 { self.amount }
	pub fn name(&self) -> &String { &self.name }
}