#(optional) re-calculate by EEA (397^-1)  in the group modulo 2357, with the tabular method given on page 19.

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
see code


#(1.11a) For positive a,b elem Z  suppose there exists u,v  satisfying au + bv = 1. Prove gcd(a,b) =1.
a, b ∈ Z
a >= b >> a = qb + r :: 0 <= r < b
if gcd(a,b) = d >> a = s(d), b=t(d)
s(d) = q(t(d)) + r
s(d)-q(t(d)) = r >> f(d) = r
>> gcd(a,b) = gcd(b,r) 
b = q'r + r'
>>gcd(b,r) = gcd(r, r') >> gcd(r't-2, r't-1) >> 1 by definition >> r't-1 = r't-1*q't-1 + rt :: rt = 1

a    = b * q  + r
ri-1   = ri * qi + ri+1
//each iteration of Euclid's can be defined recusively
ri + 1 = (ri -1) - (ri)*q
//the remainder term can be defined in terms of the (ri-1) and (ri) terms => aka, a and b terms
...
//and so on recursively until rn+1 = 1 => aka gcd = 1
rn+1 = 1 = (rn-1) - rn*q'
>>rn+1 = 1 = ua + vb

In other words, the remainder can be defined in terms of a and b. The remainder progressively approaches 1, by definition, through progressive rounds of the Euclidean algorithm, leading to the conculsion that 1 can be defined in terms of a and b => 1 = au + bv can be defined in terms of the remainder factor, which progressively approaches 1 through each round of the Euclidean algo in the case 


#(1.18) Suppose g^a == 1 mod m, g^b == 1 mod m. Prove that g^(gcd(a,b)) == 1 mod m.
gcd(a,b) = d && g^d == 1 mod m by definition
>>g^a = (g^nd) = (g^d)^n = (g^d)*(g^d) ... n times
>>1 * 1 * ...* 1 mod m >> g^a == 1 mod m

>>g^b = (g^pd) = (g^d)^p and so on in the same manner
>>g^b == 1 mod m
in the case that m is prime

- p28 might help get you started
- p33,63

gcd(g,m) = 1 >> gu + mv = 1
Then the ord(p) divides a & b => a & b => x(ord p) perhaps
Meaning that gcd(a,b) => a || b  => g ^ a || b = 1 mod m by definition

proposition 1.13

gcd(a,b) = d
(g^d) = 1 mod m >> gcd (g^d, m) = 1
g^a >> g^nd >> (g^d)(g^n) = 1 

# Using a program, obtain a generator for the group of integers in Z/1009Z and Z/2357Z. Both values are prime. What method did you use to check if the candidate was a generator?
z/1009z = 11
z/2357z = 2

Used fast_power algorithm alongside some basic checks. See code.
