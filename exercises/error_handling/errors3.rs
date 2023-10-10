/*
 * @Descripttion: my code for learning
 * @Author: chenggong Pan
 * @Date: 2023-10-09 07:53:20
 * @LastEditors: chenggong Pan
 * @LastEditTime: 2023-10-10 21:02:36
 */
// errors3.rs
//
// This is a program that is trying to use a completversion of the
// `total_cost` function from the previous exercise. ed It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::error::Error;
fn main() ->Result<(), Box<dyn Error>> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;
    
    if cost > tokens {
        println!("You can't afford that many!");
        Ok(())
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
        Ok(())
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
