#![allow(dead_code)]
mod alg;

fn main() {
    let digit = "10";
    let n = digit.parse::<i32>().unwrap();
    println!("{}", n + 2)
}
