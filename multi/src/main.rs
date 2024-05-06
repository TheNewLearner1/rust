use std::time::Instant;

fn main() {
    let a = vec![vec![1; 100]; 100];
    let b = vec![vec![1; 100]; 100];
    let start = Instant::now();
    let mut result = vec![vec![0; a[0].len()]; a.len()];

    for i in 0..a.len() {
        for j in 0..b[0].len() {
            for k in 0..a[0].len() {
                for l in 0..a[0].len() {
                    result[i][j] += a[i][k] * b[k][l] * a[l][j];
                }
            }
        }
    }

    let duration = start.elapsed();
    println!("{:?}", result);
    println!("Time required: {:?}", duration);
}

