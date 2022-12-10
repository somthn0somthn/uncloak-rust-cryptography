#Exercise 5.3 Consider SHA-512-n, a hash function that first runs SHA-512 and then outputs only the first  bits of the result. Write a program that uses a birthday attack to find and output a collision on SHA-512-n, where n is a multiple of 8 between 8 and 48. Your program may use an existing cryptography library. Time how long your program takes when n is 16, averaged over five runs for each  How long would you expect your program to take for SHA-512-256? For SHA-512?

see code in ./sha512

For the code shown, n = 16 was benchmarked at 2.6530 s per computation. However, this brute force approach over this reduced bit space only takes a portion of the time to calculate as a birthday attack on the full result.

For instance, sha512 = provides a security measure of 2^256 due to the birthday effect. This represents (2^16)(2^240) => (2.6530 s)(2^240) for sha512, which is a very large number, and (2.6530 s)(2^112) for sha256.

#Exercise 5.4 Let SHA-512-n be as in the previous exercise. Write a program that finds a message M that hashes to the following value under SHA-512-16 (in hex): 3D 4B. How many tries would you expect the algorithm to need? Running the algorithm 5 times, How many tries did it take on average?

If each attempt is random => (2^16) = 65536 tries
If duplicates are not permitted => 32768

Code in ./sha512 allows duplicate attempts (e.g. each attempt is random)
Attempts = [11270, 94360, 25304, 201232, 60381]
Avg =78509.4


#With command line tools or Criterion, benchmark the blake3 hash (default is 256 bit output), and compare it to benches of SHA3-256 and SHA-256 (when written without a number, SHA is assumed to be SHA2).

sha3_256 = 1.1822 µs
sha-256 = 634.58 ns
blake3 = 167.98 ns (an improvement of -73.961%)

