use crate::models::income::Income;
use crate::models::member::Member;
use colored::Colorize;
use std::io;

pub fn get_income(mem: &mut Member) {
    loop {
        println!("Enter income name or 'q' to quit");

        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        if name.trim() == "q" {
            let input = Income::new(String::from("Last income line"), 0.0);
            mem.add_income(input);
            return;
        }

        println!("Enter income amount");

        let mut amount = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read line");

        let input = Income::new(
            name.trim().to_string(),
            amount.trim().parse::<f32>().unwrap(),
        );

        mem.add_income(input);
    }
}
