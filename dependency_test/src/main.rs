use rust_lib_test::add;

extern crate rust_lib_test;

fn main() {
    let result = add(3, 4);

    println!("3 + 4 = {}", result);
}
