use std::cell::RefCell;

fn main() {
    let r = RefCell::new(0);
    if *r.borrow() == 0 { r.replace(1); }
    println!("{:?}", r);
    println!("{:?}", r.borrow());

}
