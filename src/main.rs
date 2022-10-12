#![allow(dead_code)]
#![allow(non_snake_case)]

mod models;
mod printer;
mod retrieve_input;
use colored::Colorize;
use models::member::Member;
use retrieve_input::collect_expenses;
use retrieve_input::collect_income;
use std::io;

fn main() {
    println!("{}", Colorize::green("Please enter your user name:"));

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let mut member = Member::new(name);

    collect_income::get_income(&mut member);
    collect_expenses::get_expenses(&mut member);

    printer::print_on_cli(&member);
}
