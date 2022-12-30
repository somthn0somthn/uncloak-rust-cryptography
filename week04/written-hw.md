Justify or disqualify each of the following schemes, with message , tag , and ciphertext.

///SSH - Mac and enc

1a) 
t = MAC(m) c = E(m), send (c,t) \
reordering attack \
{t1 .. tn } = MAC {m1 .. mn}\
c'          = E(m') where (c', m') ∈ {(c,m)} \
t'          = MAC (m') where (t', m') ∈ {(t,m) } \
(m', t') is valid \
    
1a') t[0..n] = m[len-n..len] (e.g. mac outputs a few bits of plain text)
    |m'| < |m| => |x|^(L - n) (the remainder of message unencrypted is shorter)
    c = E'(m') = E'(m) > E(m') = E(m) (a hit is now possible with few guesses reducing cost of bruted forcing)
    c = c' is considered valid
   
MAC-then-encrypt

1b) t     = MAC(m)    c     = E ( m || t ) send c [SSL]
    ~ padding attack
    c' = E(m'[0...m(n+1)] || t'[t[1] ..t[n]])
    c' = c
    c' is considered valid
    

Encrypt-then-mac 
1c) t = MAC(c)    c = E (m) send (c, t) [IPsec]
    -always provides authenticated encryption if CPA secure as long as c -> t is one-to-one


2) TLS Handshake Security

a) The client/server hello message, among other things, establishes, or re-establishes, a session Id which continues a previous session with the client. This would help prove that an unauthorized party has not intervened in the communication while also fighting against replay attacks.

b) SSL Certificate and digital signature verification  is used to prevent Man-In-The-Middle attacks increasing the integrity and authenticity of the transmission by allowing the client to verify the server. However, since the client must rely on a third-party CA for certificate verification, there are likely some weaknesses involved.

c) The symmetric key exchange is the basis of confidentiality and other desirble characteristics of the TLS handshake. For example, seeds values (i.e. server and client random values)are exchanged to generate secret keys, mututal, which the client can verify via the CA. In the case DHE-RSA is used client and server can ensure that malicious code has not been injected. Furthermore, DHE ensures that recorded TLS transmissions cannot be decrypted in the future.

d) The "client/server handshake finished" messages following the initial secret exchange also ensure that session keys on both ends have been derived correctly and the two parties can communicate securely. Pointing to confidentiality and authenticity of their communication so far and moving forward.




    
