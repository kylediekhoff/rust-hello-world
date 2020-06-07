fn main() {
    let three_multiple = sum_multiple(3, 999);
    let five_multiple = sum_multiple(5, 999);
    let fifteen_multiple = sum_multiple(15, 999);
    print!("final total: {}", three_multiple + five_multiple - fifteen_multiple);
}

fn sum_multiple(multiple: i32, n: i32) -> i32 {
    let n_multiple = n - (n % multiple);
    let n_multiplier = n_multiple / multiple;
    let factor = (n_multiplier as f32 + 1.0) / 2.0;
    let total = factor * n_multiple as f32;
    print!("n_multiple: {}\n", n_multiple);
    print!("n_multiplier: {}\n", n_multiplier);
    print!("factor: {}\n", factor);
    print!("total: {}\n", total as i32);
    total as i32
}
