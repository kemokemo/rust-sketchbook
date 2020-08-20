use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    // If you uncomment the code block here,
    // 'LSP: mismatched types expected `i32`, found `u16`'
    // warnings show here by Language Server features.
    // if a < b {
    //     println!("Ten is less than one hundred.");
    // }

    if a < (b as i32) {
        println!("Ten is less than one hundred.");
    }

    if a < b.try_into().unwrap() {
        println!("Ten is less than one hundred.");
    }
}
