use std::io;

fn main() {
    println!("Find the factorial of any unsigned integer!");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = number.trim().parse().expect("Please enter an unsigned Integer!");

    println!("Calculating the factorial of {number}...");
    // factorial(number);
    println!("{}", factorial2(number));
}

// METHOD 1
// fn factorial(mut a: u32) {
//     if a <= 1 {
//         return println!("The factorial is 1");   
//     }
//     let mut b: u32 = a - 1;
//     while b > 1 {
//         a = a * b;
//         b -= 1;
//     }
//     println!("The factorial is {a}");
// }

// METHOD 2
fn factorial2(num: u32) -> u32 {
    if num <= 1 {
      return 1;
    }
  
    num * factorial2(num - 1)
  }