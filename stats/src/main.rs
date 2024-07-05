use std::io::stdin;
fn main() {
    struct value {
        first: i32,
        last: i32,
        mid: i32,
        frequency: i32,
    }

    println!(" number of iternation ");
    let mut iternation = String::new();
    stdin().read_line(&mut iternation).unwrap();
    println!("start point");
    iternation = iternation.to_string();
    let mut point = String::new();
    stdin().read_line(&mut iternation).unwrap();

    println!("difference ");
    let mut dif = String::new();
    stdin().read_line(&mut iternation).unwrap();
    let mut x = 1;
    while x <= iternation {}
}
fn mean_deviation() {}
