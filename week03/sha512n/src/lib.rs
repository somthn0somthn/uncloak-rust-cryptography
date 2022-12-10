use sha2::{Sha256, Sha512, Digest};
use rand::Rng;
use generic_array::{GenericArray, typenum::{U1, U2, U3, U4}};
use rustc_serialize::hex::FromHex;
use blake3::*;

pub fn question1(hasher: &mut Sha512) {
    //let mut hasher: Sha512 = Sha512::new();
    let mut hasher = hasher.clone();
    hasher.update(b"hello world");
    let result =  hasher.finalize();
    let n = 2;
        
    let mut rng = rand::thread_rng();
    
    //array of u8 => one u8 int => 8-bit
    let mut array: GenericArray<u8, U2> = GenericArray::default();
    let mut result2: &[u8] = result.as_slice();
    result2 = &result2[0..n];

    loop {
        for i in 0..n {
            array[i] = rng.gen();
        }
        if array.as_slice() == result2 {
            println!("array is {:?}", array);
            println!("result is {:?}", result2);
            break
        } else {
            println!("array {:?} result2 {:?}", array, result2);
            println!("try again");
            continue
        }
    }

}

pub fn question2() {
    let hex_str = "3D4B";
    let bytes = hex_str.from_hex().unwrap();
    println!("{:?}", bytes);

    let mut rng = rand::thread_rng();
    let mut length: u8 = rng.gen();
    let mut byte_string: Vec<u8>;

    let mut counter = 0;
    

    loop {
        counter += 1;
        length = rng.gen();
        byte_string = Vec::with_capacity(length as usize);

        for _ in 0..length {
            byte_string.push(rng.gen());
            //println!("byte_string in for {:?}", byte_string);
        }

        let mut hasher = Sha512::new();
        hasher.update(&byte_string);
        let result = hasher.finalize();
        let result: &[u8] = result.as_slice();
        let result = &result[0..2];
        println!("result {:?}", result);
        println!("bytes {:?}", bytes);


        if result[0..2] == bytes[0..2] {
            println!("MATCH: result {:?} bytes {:?}", result, bytes);
            println!("final count was {counter}");
            break
        } else {
            println!("try again: result {:?} bytes {:?}", result, bytes);
            continue
        }
    }
}

pub fn question3<T: Digest + Clone> (hasher: &mut T) {
    let data = b"bubblewrap";
    let mut hasher = hasher.clone();
    hasher.update(data);
    &hasher.finalize();
}

pub fn question3a (hasher: &mut Hasher) {
    let data = b"bubblewrap";
    let mut hasher = hasher.clone();
    hasher.update(data);
    &hasher.finalize();
}
