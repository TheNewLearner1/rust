fn main() {
    let mut listo: [i32; 5] = [221, 466, 110, 945, 864];
    let max_lenth: usize = listo.len() - 1;
    let mut complete: usize = 0;
    let mut left: usize = 0;
    let mut value = listo[complete];
    while complete <= max_lenth {
        while left < complete {
            left = complete;
            if listo[left] < value {
                value = listo[left];
            }
            left += 1;
        }
        listo[complete] = value;
        complete += 1;
        if complete > max_lenth {
            break
        }
        value = listo[complete];
    }
    for text in listo {
        print!("{text}, ");
    }
}
