pub fn gen_keystream(key: &str, length: u32) -> String {
    if key.len() >= length as usize {
        let mut new_key = key;
        let index = length as usize;
        new_key = &new_key[0..index];
        return new_key.to_string()
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
    let key_stream = gen_keystream(key, plaintext.len() as u32).to_uppercase();

    
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
    let key_stream = gen_keystream(key, ciphertext.len() as u32).to_uppercase();

    let vec_cipher: Vec<_> = ciphertext.chars().map(|c| u32::from(c)).collect();
    let vec_key: Vec<_> = key_stream.chars().map(|c| u32::from(c)).collect();

    let result: String = vec_cipher.iter()
        .zip( vec_key.iter())
        .map(|(x,y)| zipper_decrypt(*x, *y))
        .map(|i| char::from(i as u8))
        .collect();    
    
    result
}


//////////////////////////////////////////////
/// 
/// punctuation not tested for
/// text only filtered for case

#[test]
fn attack_at_dawn() {
    let text = String::from("attackatdawn").to_uppercase();
    let key = "lemon";
    let cipher_text = encrypt(&text, key);
    let decipher_text = decrypt(&cipher_text, key);
    assert_eq!(text, decipher_text);
}

#[test]
fn dont_tell_anyone() {
    let text = String::from("donttellanyone").to_uppercase();
    let key = "cat";
    let cipher_text = encrypt(&text, key);
    let decipher_text = decrypt(&cipher_text, key);
    assert_eq!(text, decipher_text);
}

#[test]
fn happy_days() {
    let text = String::from("happydaysarehereagain").to_uppercase();
    let key = "aaaaa";
    let cipher_text = encrypt(&text, key);
    let decipher_text = decrypt(&cipher_text, key);
    assert_eq!(text, decipher_text);
}

#[test]
fn key_of_zs() {
    let text = String::from("itsabeautifulday").to_uppercase();
    let key = "zzzz";
    let cipher_text = encrypt(&text, key);
    let decipher_text = decrypt(&cipher_text, key);
    assert_eq!(text, decipher_text);
}

#[test]
fn constant_text() {
    let text = String::from("aaaaaaaa").to_uppercase();
    let key = "uncle";
    let cipher_text = encrypt(&text, key);
    let decipher_text = decrypt(&cipher_text, key);
    assert_eq!(text, decipher_text);
}

#[test]
fn constant_text_z() {
    let text = String::from("zzzzzzzzzzzzzzz").to_uppercase();
    let key = "aunt";
    let cipher_text = encrypt(&text, key);
    let decipher_text = decrypt(&cipher_text, key);
    assert_eq!(text, decipher_text);
}


#[test]
fn really_long_text() {
    let text = String::from("LoremipsumdolorsitametconsecteturadipiscingelitseddoeiusmodtemporincididuntutlaboreetdoloremagnaaliquaUtenimadminimveniamquisnostrudexercitationullamcolaborisnisiutaliquipexeacommodoconsequatDuisauteiruredolorinreprehenderitinvoluptatevelitessecillumdoloreeufugiatnullapariaturExcepteursintoccaecatcupidatatnonproidentsuntinculpaquiofficiadeseruntmollitanimidestlaborum").to_uppercase();
    let key = "secrets";
    let cipher_text = encrypt(&text, key);
    let decipher_text = decrypt(&cipher_text, key);
    assert_eq!(text, decipher_text);
}

#[test]
fn really_long_key() {
    let text = String::from("jpokmssrwjlazz").to_uppercase();
    let key = "LoremipsummetconsecteturadipiscingelitseddoeiusmodtemporincididuntutlaboreetdoloremagnaaliquaUtenimadminimveniamquisnostrudexercitationullamcolaborisnisiutaliquipexeacommodoconsequatDuisauteiruredolorinreprehenderitinvoluptatevelitessecillumdoloreeufugiatnullapariaturExcepteursintoccaecatcupidatatnonproidentsuntinculpaquiofficiadeseruntmollitanimidestlaboru";
    let cipher_text = encrypt(&text, key);
    let decipher_text = decrypt(&cipher_text, key);
    assert_eq!(text, decipher_text);
}

#[test]
fn really_long_text_and_key() {
    let text = String::from("rxfqgfmijapvfrahyvjbkrvrlngwdrqqsjzilbfdwhuhphtlphrbqsxaxsdivxmbhknvridnlxxvgorbhjaabdxcjridczqsumctefntukkvsjvmbikplirqtresjtyvhtkytcjzmcvcwbyufbmnwmrxzviyyjjxvecqpqkrzxnigycqjjvbxdmeqdjpipvdzcgeyenopfpnzsxzkhtdnlctonqlellwdhpsijfrukoqcrkxmwjpokmssrwjlazz").to_uppercase();
    let key = "LoremipsumdolorsitametconsecteturadipiscingelitseddoeiusmodtemporincididuntutlaboreetdoloremagnaaliquaUtenimadminimveniamquisnostrudexercitationullamcolaborisnisiutaliquipexeacommodoconsequatDuisauteiruredolorinreprehenderitinvoluptatevelitessecillumdoloreeufugiatnullapariaturExcepteursintoccaecatcupidatatnonproidentsuntinculpaquiofficiadeseruntmollitanimidestlaborum";
    let cipher_text = encrypt(&text, key);
    let decipher_text = decrypt(&cipher_text, key);
    assert_eq!(text, decipher_text);
}

#[test]
fn abc_text_rand_letter_key1() {
    let text = String::from("abcdefghijklmnopqrstuvwxyz").to_uppercase();
    let key = "jhkfvybuywqunkwmctjktbxaka";
    let cipher_text = encrypt(&text, key);
    let decipher_text = decrypt(&cipher_text, key);
    assert_eq!(text, decipher_text);
}

#[test]
fn abc_text_rand_letter_key2() {
    let text = String::from("abcdefghijklmnopqrstuvwxyz").to_uppercase();
    let key = "qmfuyqjhkgiantjjiewyvlrpos";
    let cipher_text = encrypt(&text, key);
    let decipher_text = decrypt(&cipher_text, key);
    assert_eq!(text, decipher_text);
}

#[test]
fn abc_text_rand_letter_key3() {
    let text = String::from("abcdefghijklmnopqrstuvwxyz").to_uppercase();
    let key = "wqsexvljipeyaoqidgsnyidgat";
    let cipher_text = encrypt(&text, key);
    let decipher_text = decrypt(&cipher_text, key);
    assert_eq!(text, decipher_text);
}

#[test]
fn rand_letter_text_abc_key1() {
    let text = String::from("ilofivviatrvarfaostakaixof").to_uppercase();
    let key = "abcdefghijklmnopqrstuvwxyz";
    let cipher_text = encrypt(&text, key);
    let decipher_text = decrypt(&cipher_text, key);
    assert_eq!(text, decipher_text);
}

#[test]
fn rand_letter_text_abc_key2() {
    let text = String::from("sokoltshrgbqtnbapicogpmudt").to_uppercase();
    let key = "abcdefghijklmnopqrstuvwxyz";
    let cipher_text = encrypt(&text, key);
    let decipher_text = decrypt(&cipher_text, key);
    assert_eq!(text, decipher_text);
}

#[test]
fn rand_letter_text_abc_key3() {
    let text = String::from("ogueqgmnldtyoeucrpbkhuxazn").to_uppercase();
    let key = "abcdefghijklmnopqrstuvwxyz";
    let cipher_text = encrypt(&text, key);
    let decipher_text = decrypt(&cipher_text, key);
    assert_eq!(text, decipher_text);
}