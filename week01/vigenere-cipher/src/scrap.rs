/* pub fn gen_keystream(key: &str, length: u32) -> String {
    if key.len() >= length as usize {
        let mut new_key = key.to_owned();
        let index = length as usize;
        new_key = new_key[0..index].to_owned();
        return new_key
    };
    let mut keystream = key.to_string();
    let mut n: u32 = key.len() as u32;
    while n < length {
        let letter = n as usize % key.len();
        let char = key.chars().nth(letter).unwrap();
        keystream.push(char);
        n += 1;
    }
    keystream
}

pub fn zipper_encrypt(word_int: u32, key_int: u32) -> u32 {
    let result = (word_int + (key_int - 65)).rem_euclid(90);
    if result < 65 {
        return result + 64
    }
    result
}

pub fn zipper_decrypt(cipher_int: u32, key_int: u32) -> u32 {
    let result = cipher_int - (key_int - 65);
    if result < 65 {
        return 91 - (65 - result)
    }
    result
}


pub fn encrypt(plaintext: &str, key: &str) -> String {
    let mut plaintext = plaintext.to_string();
    let text_length: u32 = plaintext.len() as u32;
    let mut key_stream = gen_keystream(key, text_length);
    println!("key_stream is {}", key_stream);

    plaintext = plaintext.to_uppercase();
    let vec: Vec<_> = plaintext.chars().map(|c| u32::from(c)).collect();
    println!("plaintext vec is {:?}", vec);

    key_stream = key_stream.to_uppercase();
    let vec2: Vec<_> = key_stream.chars().map(|c| u32::from(c)).collect();
    println!("key_stream vec2 is {:?}", vec2);

    let result: Vec<_> = vec.iter()
        .zip( vec2.iter())
        .map(|(x,y)| zipper_encrypt(*x, *y))
        .collect();
    
    println!("restult {:?}", result);

    let mut final_result: String = result.iter()
        .map(|i| char::from(*i as u8)).collect();

    
    println!("final {:?}", final_result);

    final_result
}

pub fn decrypt(ciphertext: &str, key: &str) -> String {
    let mut ciphertext = ciphertext.to_string();
    let text_length: u32 = ciphertext.len() as u32;
    let mut key_stream = gen_keystream(key, text_length);
    println!("decrypt key_stream is {}", key_stream);

    ciphertext = ciphertext.to_uppercase();
    let vec: Vec<_> = ciphertext.chars().map(|c| u32::from(c)).collect();
    println!("ciphertext vec is {:?}", vec);

    key_stream = key_stream.to_uppercase();
    let vec2: Vec<_> = key_stream.chars().map(|c| u32::from(c)).collect();
    println!("key_stream vec2 is {:?}", vec2);

    let result: Vec<_> = vec.iter()
        .zip( vec2.iter())
        .map(|(x,y)| zipper_decrypt(*x, *y))
        .collect();
    
    println!("restult {:?}", result);

    let mut final_result: String = result.iter()
        .map(|i| char::from(*i as u8)).collect();

    
    println!("final {:?}", final_result);

    final_result
} */