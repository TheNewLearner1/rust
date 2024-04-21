fn main() {
    let mut listoo = Vec::with_capacity(99_999_999);

    for i in 1..100_000_000 {
        listoo.push(i);
    }
    let target: i32 = 2;
    let mut x: i32 = 0;
    while x < 9 {
        x += 1;
    }
    let mut right: i32 = x;
    let mut left: i32 = 0;
    let index: i32 = binary_search(&listoo, left, right, target);
    println!("index {}", index);
}
fn binary_search(listo: &[i32], mut left: i32, mut right: i32, target: i32) -> i32 {
    if left > right {
        return -1;
    }
    let mut mid: usize = ((right + left) / 2) as usize;
    if listo[mid] == target {
        return mid as i32;
    } else if listo[mid] < target {
        left = mid as i32 + 1;
    } else {
        right = mid as i32 - 1;
    }
    println!("going to binary search")
    binary_search(listo, left, right, target)
}
