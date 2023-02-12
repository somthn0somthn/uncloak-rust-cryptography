# TLS 1.3 implements session resumption, allowing previously connected parties to re-use a previously used shared secret key. Does the session resumption protocol obtain forward secrecy? (Not answered in the book, use the internet).

So, forward secrecy in this context depends on whether there is a time specification defining traffic key validity - i.e. a factor that limits access to past data. Session resumption in in TLS allows a client and server to resume a session and bypass the need for a new, complete handshake resulting in faster connection times. This possible via PSKs, and PSK identity material is established in the initial handshake. 

Subsequent PSKs are established out of band and replace session IDs and session tickets which are obsolete in 1.3. Session resumption occurs via the client using these subsequent PSKs. 

PSKs are used with (EC)DHE to provide forward secrecy by creating keys useful for only the current session - the "ephemeral" part of (EC)DHE. Keys here are ephemeral and need to be recomputed via use of the PSK along a key schedule. The resumption PSK has been designed so that the resumption master secret computed by connection N, and needed for connection N+1, is separate from traffic keys.

Note, TLS 1.3 mandates forward secrecy for all TLS sessions, according to the IETF spec. 




# List the checks performed in the protocol in chapter 14 on page 235. Could this protocol be vulnerable to a Man-in-the-Middle attack?

1) Client sends the minimal size of p and a nonce

2) Server verifies requirements and  chooses (p,q,g), g^x, an authentication of this, and sends them to the client.

2) Server checks verifications and session requirements, and sends g^y and its authentication to the client

3) Client checks each. Both hash the established key  (g^xy) to generate the session key


Yes, MITM acts as the server (against the client)first, and then the client (against the server). Throughout the communication, it operates in this middle area, conveying dec/enc/re-encrypted information exchanged between the parties.


# Using the internet, find at least one method used for nonce-generation, besides a counter. Describe this method for nonce generation in the Discord study-group-main-channel chat.

Timestamp-based nonce are generated based on the current time or by using a time-based value. Like a counter, it increases monotically. In general, it seems currently that timestamps are often used alongside nonces as an authentication technique. On the other hand, it has been suggested by at least [1993](http://merlot.usc.edu/cs530-s05/papers/Neuman93a.pdf) that timestamp-based nonces by themselves are not necessarily secure, albeit potentially useful against replay attacks. 

According to this paper, timstamps can be useful in limiting the number of messages in an authentication scheme (esp Kerberos), allowing for one-way authentication in some cases.

Yet, it appears timestamp-based nonces are not common currently, if  used at all. Some drawbacks include:
* server must record timestamps received
* usage naturally relies on synchronized clocks => seems trusted party often used, increasing security concerns
* recovering from a post-dated clock is difficult
