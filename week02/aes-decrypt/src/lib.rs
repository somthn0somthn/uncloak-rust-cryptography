/* use openssl::symm::{Cipher, Mode};
use openssl::symm::Crypter;
use openssl::error::ErrorStack;
use hex::encode;
use rustc_serialize::hex::ToHex;


fn decryptor(key: &[u8], text: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_256_ecb();
    let mut output: Vec<u8> = vec![0x0; 32];
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, None).unwrap();
    crypter.pad(false);
    crypter.update(text, &mut output).unwrap(); 

    let plaintext = &output[0..16];
    plaintext.to_owned()
}


fn encryptor(key: &[u8], text: &[u8]) -> Vec<u8> {
   let cipher = Cipher::aes_256_ecb();
    let mut output: Vec<u8> = vec![0x0; 32];
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, None).unwrap();
    crypter.pad(false);
    crypter.update(text, &mut output).unwrap(); 

    let plaintext = &output[0..16];
    plaintext.to_owned()
}

#[test]
fn test_chapter3question8() {
    let key = b"\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01";
    let text = b"\x53\x9B\x33\x3B\x39\x70\x6D\x14\x90\x28\xCF\xE1\xD9\xD4\xA4\x07";
    let plaintext = decryptor(key, text);
    let result = encryptor(key, &plaintext);
    assert_eq!(text.to_hex(), result.to_hex());
}

 */