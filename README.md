# [Cryptopals challenges](https://www.cryptopals.com/)

## Tests
I decided to organize the challenges by tests, so you can easily filter them using cargo.
For example, you can type `cargo test set01` to run all set01 challenges's tests, or `cargo test ch1` to run only challenge 1's test. 

## Actual crypto code
While doing these challenges, i decided to make my own ctf crypto library, so most of the crypto code is not in this repo.
You can find it here : https://git.antoxyde.fr/Antoxyde/cryptoctf
This repo only contains the use of this library to solve cryptopals challenges.

## Stage 1: Basics

- [x] 01. Convert hex to base64
- [x] 02. Fixed XOR
- [x] 03. Single-byte XOR cipher
- [x] 04. Detect single-character XOR
- [x] 05. Implement repeating-key XOR
- [x] 06. Break repeating-key XOR
- [x] 07. AES in ECB mode
- [x] 08. Detect AES in ECB mode

## Stage 2: Block Crypto

- [x] 09. Implement PKCS#7 padding
- [x] 10. Implement CBC mode
- [x] 11. An ECB/CBC detection oracle
- [x] 12. Byte-at-a-time ECB decryption (Simple)
- [x] 13. ECB cut-and-paste
- [x] 14. Byte-at-a-time ECB decryption (Harder)
- [x] 15. PKCS#7 padding validation
- [x] 16. CBC bitflipping attacks

## Stage 3: Block & Stream Crypto

- [~] 17. The CBC padding oracle
- [~] 18. Implement CTR, the stream cipher mode
- [ ] 19. Break fixed-nonce CTR mode using substitions
- [ ] 20. Break fixed-nonce CTR statistically
- [x] 21. Implement the MT19937 Mersenne Twister RNG
- [ ] 22. Crack an MT19937 seed
- [ ] 23. Clone an MT19937 RNG from its output
- [ ] 24. Create the MT19937 stream cipher and break it

## Stage 4: Stream Crypto and Randomness

- [ ] 25. Break "random access read/write" AES CTR
- [ ] 26. CTR bitflipping
- [ ] 27. Recover the key from CBC with IV=Key
- [ ] 28. Implement a SHA-1 keyed MAC
- [ ] 29. Break a SHA-1 keyed MAC using length extension
- [ ] 30. Break an MD4 keyed MAC using length extension
- [ ] 31. Implement and break HMAC-SHA1 with an artificial timing leak
- [ ] 32. Break HMAC-SHA1 with a slightly less artificial timing leak

## Stage 5: Diffie-Hellman and Friends

- [ ] 33. Implement Diffie-Hellman
- [ ] 34. Implement a MITM key-fixing attack on Diffie-Hellman with parameter injection
- [ ] 35. Implement DH with negotiated groups, and break with malicious "g" parameters
- [ ] 36. Implement Secure Remote Password (SRP)
- [ ] 37. Break SRP with a zero key
- [ ] 38. Offline dictionary attack on simplified SRP
- [ ] 39. Implement RSA
- [ ] 40. Implement an E=3 RSA Broadcast attack

## Stage 6: RSA and DSA

- [ ] 41. Implement unpadded message recovery oracle
- [ ] 42. Bleichenbacher's e=3 RSA Attack
- [ ] 43. DSA key recovery from nonce
- [ ] 44. DSA nonce recovery from repeated nonce
- [ ] 45. DSA parameter tampering
- [ ] 46. RSA parity oracle
- [ ] 47. Bleichenbacher's PKCS 1.5 Padding Oracle (Simple Case)
- [ ] 48. Bleichenbacher's PKCS 1.5 Padding Oracle (Complete Case)

## Stage 7: Hashes

- [ ] 49. CBC-MAC Message Forgery
- [ ] 50. Hashing with CBC-MAC
- [ ] 51. Compression Ratio Side-Channel Attacks
- [ ] 52. Iterated Hash Function Multicollisions
- [ ] 53. Kelsey and Schneier's Expandable Messages
- [ ] 54. Kelsey and Kohno's Nostradamus Attack
- [ ] 55. MD4 Collisions
- [ ] 56. RC4 Single-Byte Biases
