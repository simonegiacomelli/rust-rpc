fn main() {
    let a = Box::new(Box::new(Box::new(5)));
    let b = plus_one(&a) + 37;
    println!("{}", b)
}

fn plus_one(n: &i32) -> i32 { n + 1 }