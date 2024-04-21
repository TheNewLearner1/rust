fn main() {
    let mut _x: i32 = 10;
    if _x == 10 {
        println!("{_x}");
    }
    let tup: (i32, i32, i32) = (1, 2, 3);
    let xo = tup.0;
    //why
    println!("the value of y is {xo}");
}
