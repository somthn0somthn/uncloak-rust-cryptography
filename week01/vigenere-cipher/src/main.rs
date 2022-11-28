pub fn gen_keystream(key: &str, length: u32) -> String {
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
    let mut result = (word_int + (key_int - 65)).rem_euclid(91);
    if result < 65 {
        result = result + 65
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
    let plaintext = plaintext.to_string().to_uppercase();
    let text_length: u32 = plaintext.len() as u32;
    let key_stream = gen_keystream(key, text_length).to_uppercase();
    println!("key_stream {}", key_stream);

    
    let vec_plain: Vec<_> = plaintext.chars().map(|c| u32::from(c)).collect();    
    let vec_key: Vec<_> = key_stream.chars().map(|c| u32::from(c)).collect();

    let result: String = vec_plain.iter()
        .zip( vec_key.iter())
        .map(|(x,y)| zipper_encrypt(*x, *y))
        .map(|i| char::from(i as u8))
        .collect();
    
    result
}

pub fn decrypt(ciphertext: &str, key: &str) -> String {
    let ciphertext = ciphertext.to_string().to_uppercase();
    let text_length: u32 = ciphertext.len() as u32;
    let key_stream = gen_keystream(key, text_length).to_uppercase();

    let vec_cipher: Vec<_> = ciphertext.chars().map(|c| u32::from(c)).collect();
    let vec_key: Vec<_> = key_stream.chars().map(|c| u32::from(c)).collect();

    let result: String = vec_cipher.iter()
        .zip( vec_key.iter())
        .map(|(x,y)| zipper_decrypt(*x, *y))
        .map(|i| char::from(i as u8))
        .collect();    
    
    result
}

fn main () {
    let first =encrypt("abcdefghijklmnopqrstuvwxyz", "dxzybjnyuzgenpdxzybjnyuzgenp");
    println!("first {}", first);

    let second = decrypt(&first, "dxzybjnyuzgenpdxzybjnyuzgenp");
    println!("second {}", second);
}


//TO-DO
//return STRING type from encryp
//implement decrypt -> return STRING ?? implement zipper_decrypt???
//roundtrip tests
//refactor