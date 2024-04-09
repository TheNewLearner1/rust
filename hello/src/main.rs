fn main() {
    let mut _x: i32 = 10;
    if _x == 10 {
        println!("{_x}");
    }
    let tup: (i32, i32, i32) = (1,2,3);
    let (x,y,z) = tup;
    let xo = tup.0;
    println!("the valeu of y is {xo}");
} 
