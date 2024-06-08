fn main() {
    let mut listo: [i32, 5]  = [221, 466, 110, 945, 864];
    let mut max_lenth: usize = 0;
    for _text in listo {
        max_lenth += 1
    }
    max_lenth -= 1;

    let mut complete: usize = 0;
    let right: usize = max_lenth;
    let mut left: usize = 0;
    while complete < max_lenth {
        let mut value = listo[complete];
        while left < right {
            left = complete;
            if listo[left] < value {
                value = listo[left];
                left += 1;
            }
        }
        listo[complete] = value;
        complete += 1;
    }
    for text in listo {
        print!("{text}, ");
    }
}
