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
    let mut point: i32 = input.trim().parse().expect("Please enter a valid number");

    println!("Difference:");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let dif: i32 = input.trim().parse().expect("Please enter a valid number");
    let mut x = 1;
    let mut first: Vec<i32> = Vec::new();
    let mut second: Vec<i32> = Vec::new();
    let mut frequency: Vec<i32> = Vec::new();
    input.clear();
    let mut value: i32;
    while x <= iteration {
        println!("frequency please");
        io::stdin().read_line(&mut input).expect("frequency ");
        value = input.trim().parse().expect("integer please");
        frequency.push(value);
        first.push(point);
        point += dif;
        second.push(point);
        x += 1;
    }
}
fn mean_deviation() {}
