use crate::models::member::Member;
use colored::Colorize;

pub fn print_on_cli(member: &Member) {
    println!(
        "Income and expenses for {}:",
        Colorize::blue(member.get_name().as_str()).trim()
    );

    println!("Income:          |  Expenses:          |");
    let mut amounts = Vec::<String>::new();

    for i in 0..member.get_income().len() - 1 {
        let income = [
            "+",
            member.get_income()[i].get_amount().to_string().as_str(),
        ]
        .join("");
        let income_name = member.get_income()[i].get_name().as_str();

        amounts.push([income_name, income.to_string().as_str()].join(" : "));
    }

    for i in 0..member.get_expenses().len() - 1 {
        let expense = [
            "-",
            member.get_expenses()[i].get_amount().to_string().as_str(),
        ]
        .join("");
        let expense_name = member.get_expenses()[i].get_name().as_str();
        let expense_joining = [expense_name, expense.to_string().as_str()].join(" : ");

        if i < amounts.len() {
            amounts[i] = [amounts[i].as_str(), expense_joining.as_str()].join("       ");
        } else {
            amounts.push(expense_joining);
        }
    }

    println!("{}", Colorize::blue("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"));

    for i in 0..amounts.len() {
        if amounts[i].chars().any(|c| c == '-') && !amounts[i].chars().any(|c| c == '+') {
            println!(
                "{}",
                Colorize::bright_yellow(
                    ["~~~~~~~~~~~~~~~~~", amounts[i].as_str()]
                        .join("   ")
                        .as_str()
                )
            );
        } else {
            println!("{}", Colorize::bright_yellow(amounts[i].as_str().trim()));
        }
    }
    println!("{}", Colorize::blue("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"));
	println!("Total income     : {}", Colorize::bright_magenta(member.total_monthly_income().to_string().as_str()));
	println!("Total expenses   : {}", Colorize::red(member.total_monthly_expenses().to_string().as_str()));
	println!("Remaining balance: {}", Colorize::green(member.total_monthly_left().to_string().as_str()));

}
