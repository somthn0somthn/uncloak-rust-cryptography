#1; Let  be a plaintext and let  be the length of  in bytes. Let  be the block size of the block cipher in bytes. Explain why the following is not a good padding scheme: Determine the minimum number of padding bytes necessary in order to pad the plaintext to a block boundary. This is a number  which satisfies  and  is a multiple of . Pad the plaintext by appending  bytes, each with value .

With this scheme, it P of l(p) = b will receive no padding. This means p and p || 0 have the same padded form. Also, per the authors, "all padding rules add a minimum of one byte"

#3; Suppose you, as an attacker, observe the following 32-byte ciphertext  (in hex)

46 64 DC 06 97 BB FE 69 33 07 15 07 9B A6 C2 3D
2B 84 DE 4F 90 8D 7D 34 AA CE 96 8B 64 F3 DF 75
and the following 32-byte ciphertext  (also in hex)

51 7E CC 05 C3 BD EA 3B 33 57 0E 1B D8 97 D5 30
7B D0 91 6B 8D 82 6B 35 B7 8B BB 8D 74 E2 C7 3B.
Suppose you know these ciphertexts were generated using CTR mode with the same nonce. The nonce is implicit, so it is not included in the ciphertext. You also know that the plaintext  corresponding to  is

43 72 79 70 74 6F 67 72 61 70 68 79 20 43 72 79
70 74 6F 67 72 61 70 68 79 20 43 72 79 70 74 6F.
What information, if any, can you infer about the plaintext  corresponding
to ?

You could obtain, potentially P'. You have effectivelytl determined E(K,Nonce || i). So,

P ⊕ E(K, N once || i) = C
E(K, Nonce || i ) = C ⊕ P

Then:

P' ⊕ E(K, Nonce || i) = C'
P' = C' ⊕ E(K, Nonce || i)


#4; The ciphertext (in hex):

87 F3 48 FF 79 B8 11 AF 38 57 D6 71 8E 5F 0F 91
7C 3D 26 F7 73 77 63 5A 5E 43 E9 B5 CC 5D 05 92
6E 26 FF C5 22 0D C7 D4 05 F1 70 86 70 E6 E0 17
was generated with the 256-bit AES key (also in hex)
384 bits (-258 = 128 )
IV = 128 bits =>  first row is IV
use `pub fn aes_256_cbc() -> Cipher` from Openssl::symm::Cipher

80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
(256 bits)
using CBC mode with a random IV. The IV is included at the beginning of the ciphertext. Decrypt this ciphertext. You may use an existing cryptography library for this exercise.

#6; Let ,  be a message that is two blocks long, and let 
 be a message that is one block long. Let  be the encryption of  using CBC mode with a random IV and a random key, and let 
 be the encryption of 
 using CBC mode with a random IV and the same key. Suppose an attacker knows  and suppose the attacker intercepted and thus knows  and 
. Further suppose that, by random chance, 
. Show that the attacker can compute 
.

Implement a pair of functions: A PKCS 7 message padding function, and a padding validation function that takes a message and validates whether it has a correct padding.
