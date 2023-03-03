use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes {}

// #[derive(HelloMacro)]
// struct Muffin;

fn main() {
    Pancakes::hello_macro();
    // Muffin::hello_macro();
}