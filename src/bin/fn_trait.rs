fn double_then_f<F: Fn(u64) -> u64>(n: u64, f: F) -> u64 {
    f(n * 2)
}

fn main() {
    let example_1 = double_then_f(5, |n| n + 1);

    let dynamic_value = vec![1, 2, 3].iter().sum::<u64>();
    let example_2 = double_then_f(5, |n| n + dynamic_value);
}