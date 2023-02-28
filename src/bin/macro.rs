fn main() {
    println!("macro!");
    let v = vec1![1,2,3];
}

#[macro_export]
macro_rules! vec1 {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
                println!("{}", $x);
            )*
            temp_vec
        }
    };
}