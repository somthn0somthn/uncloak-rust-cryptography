use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;

//binary expansion, then exponential expansion
fn fast_power(g: BigInt, a: BigInt, modulus: BigInt) -> BigInt {
    if a > u64::MAX.into() {
        panic!()
    }
    if a == 0.into() {
        return BigInt::from(1);
    }
    let mut mutable: BigInt = a % &modulus - 1;
    let mut binary_expansion: Vec<u64> = Vec::new();

    let two = BigInt::from(2);
    while mutable >= BigInt::from(1) {
        let bin = &mutable % &two;
        binary_expansion.push(bin.to_u64().unwrap());
        mutable /= &two;
    }

    let mut exponential_expansion: Vec<BigInt> = Vec::new();
    for i in 0..binary_expansion.len() {
        let power: u32 = 2_u32.pow(i as u32);
        let entrant = g.pow(power) % &modulus;
        exponential_expansion.push(entrant);
    }

    binary_expansion
        .iter()
        .zip(exponential_expansion.iter())
        .filter(|(a, _)| **a == 1)
        .fold(BigInt::from(1), |acc, (_, b)| acc * b.clone())
        % modulus
}

//brute force approach, apparently Shank's algorithms and others are more efficient
//but are part of ch2 HW so will wait to implement
fn all_generators(modulus: u64) -> Vec<u64> {
    let mut generators = Vec::with_capacity(modulus as usize - 1);
    for g in 2..modulus {
        let mut residues = Vec::with_capacity(modulus as usize - 1);
        for a in 1..modulus {
            let residue = fast_power(g.into(), a.into(), modulus.into());
            if residues.contains(&residue) {
                break;
            }
            residues.push(residue);
        }
        if residues.len() == modulus as usize - 1 {
            generators.push(g);
        }
    }
    generators
}

//checking results of all_generators
fn is_generator(g: u64, modulus: u64) -> (bool, Vec<BigInt>) {
    let mut vect: Vec<BigInt> = Vec::new();
    for a in 1..modulus {
        let result = fast_power(BigInt::from(g), BigInt::from(a), BigInt::from(modulus));
        if !vect.contains(&result) {
            vect.push(result);
        } else {
            vect.sort();
            break;
        };
    }
    if vect.len() == modulus as usize - 1 {
        vect.sort();
        return (true, vect);
    } else {
        return (false, vect);
    }
}

//messing around
fn main() {
    let vect: Vec<BigInt> = Vec::new();
    //let modulus1: u64 = 1009;
    let modulus2: u64 = 2357;
    //let modulus2: u64 = 7;
    //let answer = all_generators(modulus2);
    //let answer2 = tester(modulus2);

    //println!("answer1: {:?}\nanswer2: {:?}", answer1, answer2);
    //println!("answer {:?}, length {:?}", answer, answer.len());

    let is_gen = is_generator(1511, modulus2);
    println!(
        "\n \n VECT: {:?} \n  IS_GEN: {:?} \n  LEN {}",
        is_gen.1,
        is_gen.0,
        is_gen.1.len()
    );
}
