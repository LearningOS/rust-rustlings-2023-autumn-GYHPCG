/*
 * @Descripttion: my code for learning
 * @Author: chenggong Pan
 * @Date: 2023-10-09 07:53:20
 * @LastEditors: chenggong Pan
 * @LastEditTime: 2023-10-10 18:07:20
 */
// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {//ref p 创建一个对P的不i可变引用
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
