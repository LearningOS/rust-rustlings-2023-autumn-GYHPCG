/*
 * @Descripttion: my code for learning
 * @Author: chenggong Pan
 * @Date: 2023-10-09 07:53:20
 * @LastEditors: chenggong Pan
 * @LastEditTime: 2023-10-09 21:57:18
 */
// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
