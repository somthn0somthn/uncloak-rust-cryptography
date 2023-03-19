#Compute with pen and paper, the Elgamal algorithm for Alice sending Bob a message. Use parameters:
 p=19, g=2, a=3, b=5, m=12. 
#You may choose any k. If an adversary obtained the computed ciphertext (c1,c2), and obtained access to k, how could they decrypt the ciphertext?
*kepehm  = 3
A == 8  == g^a mod p == 2^3 mod 19 
B == 13 == g^b mod   == 2^5 mod 19 

c1 == 8   ==   g^k mod p == 2^3 mod 19
c2 == 7   == m*A^k mod p == 12 * 8^3 mod 19

x    == 18 == c1^a  mod p == 8^3 mod 19
x^-1 == 18
m    == 12 == c2 * x^-1 mod p == 7 * 18 mod 19

#Compute with Shank's algorithm, on pen and paper, the discrete log of (2^x = 33 mod 83) Note that 2 is a generator, which you may check by verifying (2^41 = 82 mod 83) (check that you understand why this works).

* factors of 82 == 1,2,41,82
2^1,2^2, & 2^41 != 1 mod 83 => 2^41 mod 83 == 82 => 2 is a prime root mod 83 => |2| = 82
- choose some k :: k = 7
2^1  =  2
2^2  =  4
2^3  =  8
2^4  = 16
2^5  = 32
2^6  = 64
2^7  = 45

-then evaluate 33*2^-k, 33*2^-2k...33*2^-rk  where rk > 83 => r == 12

2^-1  = 42*1  = 42 mod 83
2^-2  = 42*42 = 21 mod 83
2^-4  = 21*21 = 26 mod 83
2^-8  = 26*26 = 66 mod 83
2^-16 = 66*66 = 40 mod 83
2^-32 = 40*40 = 23 mod 83
2^-64 = 23*23 = 31 mod 83

a) 33 * 26 * 21 * 42 mod 83 == 45
=> 2^7 mod 83 == 45 == 2^-7 mod 83


#Optional: Implement one or both of Elgamal encryption and/or Shank's algorithm.



#NOTES :: 2.1-2.4, 2.6, 2.7
-modular groups are abelian => identity, inverse, associative, commutative

- in a DH key exchange, Alice and Bob must decide on p, a large prime, and g. G should be a generator if the field Fp so that the potential values created during the calculation of the private key are as large as possible. Remember, Alice sends A, g^a, to Bob and Bob sends B, g^b, to Alice. Both then compute g^(ab) via a or b, which are secret to each party. If g is not a generator of Fp, it means that less than p-1 values are potentially the solution of g^ab. If the number of potential values of g^ab is significantly less than p, than an attacker has an easier time breaking the key exchagne, e.g. by the Pohlig-Hellman attack. 

## So, g must be a generator in Fp

-elgmal psuedo code
* choose p and g, where p is a "large" prime and g is a generator of order p/2 or greater => preferably g is a primitive root => choose p and g that are preselected by a trusted party
* choose a secret number a to act as a priv key e.g. g^a =A mod p
* compute A == g^a 
* choose ephemeral key, k, modulo p
* c1 == g^k
* c2 == mA^k => m(g^ak)
* x = c1^a => g^ka
* x^-1 => (g^ka)^-1 => requires EEA
* c2 * x^-1 = m

-see table 2.3 for diagram
