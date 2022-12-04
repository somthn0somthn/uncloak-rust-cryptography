##Chapter 3 (p. 61) 
#1; How much space would be required to store a table for an entire idealized block cipher that operates on 64-bit blocks and has 80-bit keys?
(2^64) * 64 * (2^80) = 2^150 bits

#5; Suppose you have a processor that can perform a single DES encryption or decryption operation in  seconds. Suppose you also have a large number of plaintext-ciphertext pairs for  under a single unknown key. How many hours would it take, on average, to find that  key, using an exhaustive search approach and a single processor? How many hours would it take, with a collection of  processors?2
149130
9

#6; Consider a new block cipher, DES2, that consists only of two rounds of the DES block cipher. DES2 has the same block and key size as DES. For this question you should consider the DES  function as a black box that takes two inputs, a 32-bit data segment and a 48-bit round key, and that produces a 32-bit output. Suppose you have a large number of plaintext-ciphertext pairs for DES2 under a single, unknown key. Given an algorithm for recovering the 48-bit round key for round 1 and the 48-bit round key for round 2. Your algorithm should require fewer operations than an exhaustive search for an entire 56-bit DES key. Can your algorithm be converted into a distinguishable attack against DES2?
Yes, ignoring the meet-in-the-middle attack, DES2 would take (2^56)*(2^56) = 2^112 operations to brute force. Or, you could say (2 ^ n)*(2 ^ n) = (2^2n). 

By definition above, your algorithm reduces the operations to determine the 56-bit of the first DES cipher => (2^(n-x))(2^n) => 2^(n(n-x)) which is less than (2^2n) => a successful distinguishing attack.
 

#8; Familiarize yourself with a cryptographic CLI tools. A popular open source package is OpenSSL. Using an existing cryptographic library, decrypt the following ciphertext (in hex)

	53 9B 33 3B 39 70 6D 14 90 28 CF E1 D9 D4 A4 07
with the following 256-bit key (also in hex):

	80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
	00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
using AES.

80 70 60 50 40 30 20 10 08 07 06 05 04 03 02 01

#9; Using an existing cryptography library, encrypt the following plaintext (in hex)

	29 6C 93 FD F4 99 AA EB 41 94 BA BC 2E 63 56 1D
with the following 256-bit key from problem 8, using AES. Then re-encrypt and decrypt it using a 3072-bit RSA key with GnuPG, or your choice of asymmetric crypto CLI.

80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01
#10; Write a program that experimentally demonstrates the complementation property for DES. This program should take as input a key  and a plaintext  and demonstrate that the DES complementation property holds for this key and plaintext. You may use an existing cryptography library for this exercise.

Difficulty finding reliable DES implementation