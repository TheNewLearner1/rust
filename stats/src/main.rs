use std::io::{self, Write};

fn main() {
    struct Value {
        first: i32,
        last: i32,
        mid: i32,
        frequency: i32,
    }

    println!("Number of iterations:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let iteration: i32 = input.trim().parse().expect("Please enter a valid number");

    println!("Start point:");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let point: i32 = input.trim().parse().expect("Please enter a valid number");

    println!("Difference:");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let dif: i32 = input.trim().parse().expect("Please enter a valid number");

    let mut x = 1;
    while x <= iteration {

    }
}
fn mean_deviation() {}
