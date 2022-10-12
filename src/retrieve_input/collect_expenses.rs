use crate::models::expense::Expense;
use crate::models::member::Member;
use colored::Colorize;
use std::io;

pub fn get_expenses(mem: &mut Member) {
    loop {
        println!("Enter expense name or 'q' to quit");

        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        if name.trim() == "q" {
            let input = Expense::new(String::from("Last expense line"), 0.0);
            mem.add_expense(input);
            return;
        }

        println!("Enter expense amount");

        let mut amount = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read line");
        let input = Expense::new(
            name.trim().to_string(),
            amount.trim().parse::<f32>().unwrap(),
        );
        mem.add_expense(input);
    }
}
