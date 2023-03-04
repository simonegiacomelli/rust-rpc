use hello_macro::HelloMacro;
use hello_macro_derive::{HelloMacro, sql};

#[derive(HelloMacro)]
struct Pancakes {}

// #[derive(HelloMacro)]
// struct Muffin;

fn main() {
    Pancakes::hello_macro();
    // Muffin::hello_macro();
    // let sql = sql!(SELECT * FROM posts WHERE id=1 );
    let sql = sql!(aaa * bbb ccc 123 ddd #{ciccio} (foo) /* ole */  );
    // let sql = sql!("ciccio");
}