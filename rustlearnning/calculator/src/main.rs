use std::io;

mod calc {
    pub fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }
    pub fn sub(a: i32, b: i32) -> i32 {
        return a - b;
    }
    pub fn multi(a: i32, b: i32) -> i32 {
        return a * b;
    }
    pub fn div(a: i32, b: i32) -> f32 {
        return (a as f32) / (b as f32);
    }
}

use calc::*;

fn getvaluefromuser() -> (i32, i32) {
    let mut input_one = String::new();
    println!("enter the input one: ");
    io::stdin()
        .read_line(&mut input_one)
        .expect("not a valid input");

    let number_one: i32 = input_one.trim().parse().expect("not a valid number");

    let mut input_two = String::new();
    println!("enter another number: ");
    io::stdin()
        .read_line(&mut input_two)
        .expect("not a valid input");
    let number_two: i32 = input_two.trim().parse().expect("not a valid number");

    return (number_one, number_two);
}
fn main() {
    println!("Welcome to the OG calculator");

    let (a, b) = getvaluefromuser();
    println!(
        "
    for addition press 1 \n
    for substraction press 2 \n
    for multiplication press 3 \n 
    for division press 4 \n
    "
    );
    let mut oper = String::new();
    io::stdin()
        .read_line(&mut oper)
        .expect("enter a valid operator");
    let operator_me: u8 = oper.trim().parse().expect("not valid");

    match operator_me {
        1 => println!("{}", add(a, b)),
        2 => println!("{}", sub(a, b)),
        3 => println!("{}", multi(a, b)),
        4 => println!("{}", div(a, b)),
        _ => println!("not a valid command"),
    }
}
