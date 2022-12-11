#Exercise 6.3 Suppose a and b are both one block long, and suppose the sender MACs a, b, and  with CBC-MAC. An attacker who intercepts the MAC tags for these messages can now forge the MAC for the message , which the sender never sent. The forged tag for this message is equal to , the tag for . Justify mathematically why this is true.

M(b || q)  = Ek ( q ⊕ M(b))

q = (M(b) ⊕ M(a) ⊕ b)

:= Ek (M(b) ⊕ M(b) ⊕ M(a) ⊕ b)

:= Ek (M(a) ⊕ b )

= M(a || b)   


#Exercise 6.4 Suppose message  is one block long. Suppose that an attacker has received the MAC  for a using CBC-MAC under some random key unknown to the attacker. Explain how to forge the MAC for a two-block message of your choice. What is the two-block message that you chose? What is the tag that you chose? Why is your chosen tag a valid tag for your two-block message?

The second block has to be t ⊕ m, where t is ta.

M(m || (t ⊕ m)) = Ek ((t ⊕ m) ⊕ t)
:= Ek (m) = t

#Exercise 6.5 Using an existing cryptography library, compute the MAC of the message:
4D 41 43 73 20 61 72 65 20 76 65 72 79 20 75 73 65 66 75 6C 20 69 6E 20 63 72 79 70 74 6F 67 72 61 70 68 79 21 20 20 20 20 20 20 20 20 20 20 20

#using CBC-MAC with AES and the 256-bit key:

80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01

$ echo 4D4143732061726520766572792075736566756C20696E2063727970746F677261706879212020202020202020202020 | openssl dgst -hmac 8000000000000000000000000000000000000000000000000000000000000001 -hex

#For message authentication, when would you use TupleHash? ParallelHash? KMAC?

TupleHash: For hashing tuples of input strings

ParralelHash: when you want to hash very long messages in parallel

KMAC: when you need a PRNG or a more versatile output size
