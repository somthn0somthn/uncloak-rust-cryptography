use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;

//binary expansion, then exponential expansion
fn fast_power(g: BigInt, a: BigInt, modulus: BigInt) -> BigInt {
    let mut mutable: BigInt = a;
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
    .filter(|(a, _)| **a ==1)
    .fold(BigInt::from(1), |acc, (_,b)| acc * b.clone()) % modulus
}

#[test]
fn three_to_thirtyfive() {
    let three = BigInt::from(3);
    let thirty_five = BigInt::from(35);
    let thirty_seven = BigInt::from(37);
    let expected =  BigInt::from(25);
    
    assert_eq!(fast_power(three, thirty_five, thirty_seven), expected);
}

#[test]
fn four_to_fifty() {
    let four = BigInt::from(4);
    let fifty = BigInt::from(50);
    let nineteen = BigInt::from(19);
    let expected = BigInt::from(17);

    assert_eq!(fast_power(four, fifty, nineteen), expected);
}

#[test]
fn three_to_twohundredeighteen(){
    let three = BigInt::from(3);
    let two_hundred_eighteen = BigInt::from(218);
    let thousand =  BigInt::from(1000);
    let expected = BigInt::from(489);

    assert_eq!(fast_power(three, two_hundred_eighteen, thousand), expected)
}

#[test]
fn two_forty() {
    let two_forty = BigInt::from(240);
    let two_sixty_two = BigInt::from(262);
    let fourteen = BigInt::from(14);
    let expected = BigInt::from(2);

    assert_eq!(fast_power(two_forty, two_sixty_two, fourteen), expected);
}

#[test]
fn one(){
    let one = BigInt::from(1);
    let three_fifty = BigInt::from(350);
    let eighty_five = BigInt::from(85);
    let expected = BigInt::from(1);

    assert_eq!(fast_power(one, three_fifty, eighty_five), expected);

}

#[test]
fn two_fifty_six() {
    let seven = BigInt::from(7);
    let two_fifty_six = BigInt::from(256);
    let thirteen = BigInt::from(13);
    let expected = BigInt::from(9);

    assert_eq!(fast_power(seven, two_fifty_six, thirteen), expected);
}

#[test]
fn two_twenty_three() {
    let two = BigInt::from(2);
    let two_twenty_three = BigInt::from(223);
    let three_fifty_three = BigInt::from(353);
    let expected = BigInt::from(345);

    assert_eq!(fast_power(two, two_twenty_three, three_fifty_three), expected);
}