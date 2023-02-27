use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;


//binary expansion complete, continue computing powers of g, see page 25
fn fast_power(g: BigInt, a: BigInt, modulus: BigInt) -> BigInt {
    let mut answer = a;
    let mut vec1: Vec<u64> = Vec::new();
    let two = BigInt::from(2);
    while answer >= BigInt::from(1) {
        let bin = &answer % &two;
        vec1.push(bin.to_u64().unwrap());
        answer /= &two;
        println!("answer: {:?}", answer);
    }
    println!("vec1: {:?}", vec1);
    let mut vec2: Vec<BigInt> = Vec::new();
    for i in 0..vec1.len() {
        println!("i: {i}");
        let power: u32 = 2_u32.pow(i as u32);
        let entrant = g.pow(power) % &modulus;
        println!("power: {power} entrant: {:?}", entrant);
        vec2.push(entrant);
    }
    let mut zipped: Vec<_> = vec1.iter().zip(vec2.iter()).collect();
    println!("zipped: {:?}", zipped);
    let filtered:Vec<_> = zipped.iter().filter(|(a, b)| **a == 1).collect();
    println!("filtered: {:?}", filtered);
    let mut folded: Vec<_> =  filtered.clone();
    
    let answer = folded.iter().fold(BigInt::from(1), |acc, (_,b)| acc * *b) % modulus;

    //filter
    //multiply
    println!("answer: {:?}", answer);
    answer
}