# CH9
# What is entropy? Why does a combination (eg. by XOR, or by hash-concatenation as on p145, Reseed) of two or more independent input-streams of entropy has at least entropy; that is, why a combination of entropy streams is always at least as entropic as the most entropic stream.                 Note the word independent: an adaptive adversary controlling stream  may choose a function of the other streams to control the randomness.
-Entropy is a measure of randomness - with randomness being a crucial input into generating secure cipher keys. Another way is describing entropy would be how much uncertainty is involved in, say, the generation of a 32 bit key => 2^32. The more that is known about a value, the smaller the entropy. A known value has no entropy.

-The effect of the most entropic steam will be preserved by a combination event, say an XOR. For example, if a stream is known to contain n consecutive 0s those will change in an unpredictable way determined by the most entropic stream. The unknown portion will also change unpredictably by being XORd with the most entropic stream. The end result is greater entropy, or at least equal entropy to the most entropic stream.Another way to phrase this, is that the combination of streams only increases the total number of outcomes and the uncertainty of what that outcome will be.

# Give a one-sentence explanation of the difference between a PRNG and a CSPRNG (non-correlation versus unpredictability). 
An attacker, upon seeing part of the CSPRNG output, should not be able to predict anything about the rest of the output.

# Why can a CSPRNG can be constructed from a block cipher?
-A block cipher can be used in CTR mode to generate a random stream of data => the output of the CSPRNG. A newly generated key upon serving random can be to generate further values meaning previous results will not be compromised if the state of the cipher is compromised. Also, not resetting a counter helps ensure that key values are not repeated.


# CH10

### NOTES
-random data used to seed a PRNG
-pooling entropy reduces attack vectors on PRNGs => Yarrow try to estimate entropy, which is difficult to do