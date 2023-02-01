use std::cmp::{max, min};
use std::ops::Div;
use rand::prelude::*;

fn gcd(a: u32, b: u32) -> u32 {
    let mut max = max(a,b);
    let mut min = min(a,b);
    while min != 0 {
        (min,max) = (max % min, min)
    }
    max
}

fn eea(a: u32, b: u32) -> (u32, i32, i32) {
    if b == 0 {
        let (d, x, y) = (a, 1, 0);
        return (d, x, y);
    } else {
        let (d, x1, y1) = eea(b, a % b);
        let q = a.div(b);
        let x = y1;
        let y = (x1 as i32) - (y1 as i32) * (a as i32 / b as i32);
        return (d, x, y);
    }    
}



fn main() {
    let answer = eea(397, 2357);
    println!("answer: {:?}", answer);
}
