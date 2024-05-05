fn main() {
    let mut _x: i32 = 10;
    if _x == 10 {
        println!("{_x}");
    }
    let tup: (i32, i32, i32) = (1, 2, 3);
    let xo = tup.0;
    let yo = tup.1;
    //why
    struct Person {
        name: String,
        age: i16,
        gender: String,
        iq: i32,
        fat: bool,
    }
    let sanjok = Person {
        name: String::from("Sanjok"),
        age: 16,
        gender: String::from("male"),
        iq: 78,
        fat: true
    };
    println!("the value of y is {xo}, x is {yo}");
    let str11 = String::from("hilol");
    let m = cal_len(&str11);
    println!("fuck you");
    println!("lol");
    println!("word {str11}, len {m} ");
}
    
fn cal_len(x: &str) -> usize {
    x.len()
}
