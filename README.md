Rust implementation of the quantum circuit synthesis algorithm for binary field multiplication.

### Usage
[Install rust](https://www.rust-lang.org/tools/install) and run ```cargo run -r <DEGREES>```
where ```<DEGREES>``` represents the degrees of the non-zero terms of the primitive polynomial.
For example, ```5 2 0``` represents the primitive polynomial $x^5 + x^2 + 1$.
The generated circuit will be written in the .qc format and located in  ```circuits/gf2^n_mult.qc```.
