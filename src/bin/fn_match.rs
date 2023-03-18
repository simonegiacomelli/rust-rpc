fn main() {
    let v = 42;
    fun(&v);
    fun2(&v);
}

fn fun(&x: &i32) {
    let p = x + 1;
    println!("{:?}", p)
}

fn fun2(x: &i32) {
    let p = x + 1;
    println!("{:?}", p)
}
