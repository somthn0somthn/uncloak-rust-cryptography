use std::cmp::{max, min};

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
        let q = a / b;
        let x = y1;
        let y = (x1 as i32) - (y1 as i32) * (a as i32 / b as i32);
        return (d, x, y);
    }    
}

//not testing for wrapping

#[test]
fn test_small() {
    assert_eq!(gcd(2,4), 2);
}

#[test]
fn test_primes() {
    assert_eq!(gcd(7,11), 1);
}

#[test]
fn test_same() {
    assert_eq!(gcd(3,3), 3);
}

#[test]
fn test_one() {
    assert_eq!(gcd(21,1), 1);
}

#[test]
fn zero() {
    assert_eq!(gcd(0,5), 5);
}

#[test]
fn out_of_order() {
    assert_eq!(gcd(15, 10), 5);
}

#[test]
fn is_smaller() {
    assert_eq!(gcd(15,30), 15);
}

#[test]
fn basic() {
    assert_eq!(eea(60, 48), (12, 1, -1));
}

#[test]
fn basic_rev() {
    assert_eq!(eea(48, 60), (12, -1, 1));
}

#[test]
fn fives() {
    assert_eq!(eea(10, 25), (5, -2, 1));
}

#[test]
fn small_prime() {
    assert_eq!(eea(5,7), (1,3,-2));
}

#[test]
fn hundred() {
    assert_eq!(eea(100,35), (5,-1,3))
}

fn sixes() {
    assert_eq!(eea(24, 18), (6,1,-2))
}