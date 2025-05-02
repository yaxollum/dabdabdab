This is a worker process (written in Rust) that tries to find a permutation of the numbers 0, 1, ..., 36 which results in an SHA-256 hash that starts with `dabdabdab`.

You can launch multiple worker processes by running `./launch.sh`.

---

One possible solution is `N = 24607176111991806`, which corresponds to the permutation `14 6 12 35 3 34 30 8 0 24 21 13 10 11 16 26 32 15 2 18 33 31 19 9 25 22 7 27 28 29 20 1 5 23 4 17 36`. This was found after computing over 40 billion hashes - which took several hours since the program isn't very optimized and can compute roughly 0.5 million hashes/second.
