# Description 
A simple CLI program that lists all permutations.

# Why?
Some idiot somehow lost the ordering of his private keys on mongolian basket-weaving forum i sometimes browse.

# How to install
open terminal and do:
```
git clone https://github.com/K-C-DaCosta/permutation_generator.git
cd permutation_generator
```
# Example
running:
```
cargo run --release -- -l 'a b c'
```
yields: 
```
["a", "b", "c"]
["a", "c", "b"]
["b", "a", "c"]
["b", "c", "a"]
["c", "a", "b"]
["c", "b", "a"]
```

