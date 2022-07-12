use std::io;

fn main() {
    println!("Triangle!!!\nEnter the max number: ");

    let mut max = String::new();

    io::stdin()
        .read_line(&mut max)
        .expect("Failed to read line");

    let max : u32 = max.trim().parse().expect("Enter a number");
    println!("\n\n");
    for i in 1..(max+1) {
        println!("{}", triangle_up(i));
    }
    for i in (1..(max+1)).rev() {
        println!("{}", triangle_down(i));
    }
}

fn triangle_up(num: u32) -> String {
    let mut astericks = String::with_capacity(num as usize);
    for _ in 1..(num+1) {
        astericks.push('*');
    }
    astericks
}

fn triangle_down(num: u32) -> String {
    let mut astericks = String::with_capacity(num as usize);
    for _ in (1..num).rev() {
        astericks.push('*');
    }
    astericks
}
