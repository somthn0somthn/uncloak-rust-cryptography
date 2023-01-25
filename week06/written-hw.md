# Pick one implementer of CryptoRng, and explain how the Rng generates values.
-`ChaCha8Rng` uses the ChaCha stream cipher as a PRNG. More specifically, ChaCha uses add-rotate-xor operations. Briefly, this includes
1. Add: ChaCha initializes a state vector with a set of constant values and then add the state vec's current values to itself (mod 2^n I'd imagine). This serves to mix the original state and make it unpredictable
2. Rotate: ChaCha performs a series of rotate operations on different parts of the vector which shuffles values in a predictable way, making it difficult for an attacker to predict the next value (consider the CryptoRng documentatio: "It should not be possible using polynomial-time algorithms to predict the next bit with probability significantly greater than 50%.")
3. XOR: ChaCha XORs the current state vector with a key

In the cipher, mode this process is used to creat a keystream, however here, it is used as a stream of pseudo-random numbers. Finally, the library relies on Single Instruction, Multiple Data (SIMD) intstructions to expedite the algorithm on modern hardware.

# <question 2>
- see last crypt lec of week5 at about the 4 min mark
