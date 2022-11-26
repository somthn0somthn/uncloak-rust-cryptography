# CH 1

## Q10. Describe a concrete example where improving the security of a system against one type of attack can increase the likelihood of other attacks.

As cryptocurrency funds have been increasingly moved off exchanges, out of hot wallets which can be hacked, and into more resilient cold storage devices, there has been a concurrent increase in phishing attacks aiming to trick users into divulging their wallet recovery phrase (e.g., Ledger "support staff" emailing customers for seed phrases, etc.).

# CH 2

## Q3. Consider a group of 30 people who wish to establish pair-wise secure communications using symmetric-key cryptography. How many keys need to be exchanged in total.
435

## Q4. Suppose Bob receives a messages signed using a digital signature scheme with Alice’s secret signing key. Does it prove that Alice saw the message and chose to sign.
In theory, yes, but if the operative word is "prove," and its taken literally, then the limitations inherent in the scheme prevent that from being so. A computer computes the signature and that computer could be compromised, for instance.

## Q6. Suppose a chosen-ciphertext attacker cannot recover the secret decryption key for an encryption scheme. Does this mean the encryption scheme is secure?
Most likely yes, as long as the encryption scheme is reasonably strong.


## Q7. Consider a symmetric-key cryptosystem in which cryptographic keys are randomly selected from the set of all n-bit strings. Approximately what should n be in order to provide 128 bits of security against a birthday attack.
256
