use cbc_mac::{CbcMac, Mac};
use aes::Aes256;
/* use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
 */
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, 
    generic_array::GenericArray,
};

use generic_array::typenum::consts::*;
use hex_literal::hex;
use hex::decode;
//use generic_array::{arr, GenericArray};

fn main() {
    let mut message = "4D 41 43 73 20 61 72 65 20 76 65 72 79 20 75 73 65 66 75 6C 20 69 6E 20 63 72 79 70 74 6F 67 72 61 70 68 79 21 20 20 20 20 20 20 20 20 20 20 20";
    let message = message.replace(" ", "");
    let message = hex::decode(&message).unwrap();

    //let mut key = "8000000000000000000000000000000000000000000000000000000000000001";
    //let key = key.replace(" ", "");
    //let key = decode(&key).unwrap();
    //let key: &GenericArray<u8, U32> = GenericArray::from_slice(&key);
    
    let key = hex!("8000000000000000000000000000000000000000000000000000000000000001");

    println!("message : {:?}", message);
    println!("key : {:?}",key);

    type Aaa = CbcMac<Aes256>;

    let mut mac = Aaa::new_from_slice(&key).unwrap();

    

    
}