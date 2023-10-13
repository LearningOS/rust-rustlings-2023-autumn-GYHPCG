/*
 * @Descripttion: my code for learning
 * @Author: chenggong Pan
 * @Date: 2023-10-09 07:53:20
 * @LastEditors: chenggong Pan
 * @LastEditTime: 2023-10-12 19:26:57
 */
// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!("7777");
}
