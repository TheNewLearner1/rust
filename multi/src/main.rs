use std::time::Instant;

fn main() {
    let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let b = vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]];
    let start = Instant::now();
    let mut result = vec![vec![0; a[0].len()]; a.len()];

    for i in 0..a.len() {
        for j in 0..b[0].len() {
            for k in 0..a[0].len() {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    let duration = start.elapsed();
    println!("{:?}", result);
    println!("Time elapsed: {:?}", duration);
}
