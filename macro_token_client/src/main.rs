use hello_macro_derive::HelloMacro;
use macro_token::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
