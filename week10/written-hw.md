#(optional) re-calculate by EEA (397^-1)  in the group modulo 2357, with the tabular method given on page 19.
- in other words, this is asking 397 * x == 1 (mod 2357) =>  x = (397^-1) mod 2357 => so, what is x?
- see p19, 20, 21

2357 = 397*5  + 372
397  = 372*1  + 25
372  =  25*14 + 22
25   =  22*1  + 3
22   =   3*7  + 1
3    =   1*3  + 0

    |5   |1   |14  |1   |7   |3   |
0 1 |5   |6   |89  |95  |754 |2357|
1 0 |1   |1   |15  |16  |127 |397 |

(-754) | 1603

#implement the modular Fast Powering Algorithm for big ints.
- see p24
- see Boneh Cryptography Course:: Week05 Arithmetic algorithms - near end of vid


#(1.11a) For positive a,b elem Z  suppose there exists u,v  satisfying au + bv = 1. Prove gcd(a,b) =1.
- see page 16,29,14,17
- see YT Introduction to Number theory lecture 4
- there is 

start by defining a larger than b and enter the EEA to end with gcd of 1 using symbols
#(1.18) Suppose g^a == 1 mod m, g^b == 1 mod m. Prove that g^(gcd(a,b)) == 1 mod m.
- p28 might help get you started


# Using a program, obtain a generator for the group of integers in Z/1009Z and Z/2357Z. Both values are prime. What method did you use to check if the candidate was a generator?
- see page 21
