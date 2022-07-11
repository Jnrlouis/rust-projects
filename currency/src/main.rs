use std::io;

const NAIRA_DOLLARS: f64 = 618.00;

fn main() {

    println!("Convert Curreny. Choose from the following options (type number to begin):");
    println!("(1) Naira to Dollars");
    println!("(2) Dollars to Naira");

    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let value:u32 = value.trim().parse().expect("Please enter a number");
    let choice;
    let sign;
    let converted_sign;
    match value {
        1 => {
            choice = "Naira";
            sign = "₦";
            converted_sign = "$";
            println!("Converting from Naira to Dollars.");
        },

        2 => {
            choice = "Dollars";
            sign = "$";
            converted_sign = "₦";
            println!("Converting from Dollars to Naira.");
        },

        _ => {
            println!("Choose a valid option");
            return;
        }
    };

    println!("Enter an amount to convert: ");

    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read line");
    
    let amount = amount.trim().parse::<f64>().expect("Please enter a number");
    
    println!("Naira to Dollars is 618 as at 11th July, 2022.");
    println!("{sign}{amount} is {converted_sign}{:.4}. In essence, we're forked.", convert(amount, choice));
}

fn convert (amount: f64, choice: &str) -> f64 {
    match choice {
        "Naira" => amount/NAIRA_DOLLARS,
        "Dollars" => amount * NAIRA_DOLLARS,
        _ => amount
    }
}
