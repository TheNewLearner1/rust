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
        median(&mut first, &mut second, &mut frequency);
    }
}
fn median(first: &mut Vec<i32>, second: &mut Vec<i32>, frequency: &mut Vec<i32>) -> Option<f64> {
    let total_count: i32 = frequency.iter().sum();
    let mid_point = (total_count + 1) / 2;

    let mut cumulative_freq = 0;
    let mut median_index = 0;

    for (i, &freq) in frequency.iter().enumerate() {
        cumulative_freq += freq;
        if cumulative_freq >= mid_point {
            median_index = i;
            break;
        }
    }

    if total_count % 2 == 1 {
        // Odd number of elements
        Some(first[median_index] as f64)
    } else {
        // Even number of elements
        let next_index = median_index + 1;
        if next_index < first.len() {
            Some((first[median_index] as f64 + first[next_index] as f64) / 2.0)
        } else {
            None // Error case: not enough elements
        }
    }
}
