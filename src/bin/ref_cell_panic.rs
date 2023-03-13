use std::cell::RefCell;

fn main() {
    println!("start...");
    let ref_cell = RefCell::new(1);
    let i1 = ref_cell.borrow();
    let i1 = ref_cell.borrow();
    let m = ref_cell.borrow_mut();
}