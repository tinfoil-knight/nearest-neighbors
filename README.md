# nearest-neighbors

Collection of algorithms to find the k-nearest neighbors in a vector dataset.

## Implementations

- k-D Tree
- Vantage-Point / VP Tree
- Locality Sensitive Hashing w/ Random Projection

## Usage

For compatibility, your dataset should be in this format:

```
<token_1> <value_1> <value_2> <value_3> ... <value_n>
<token_2> .  .  .  .
 .
 .
 .
 .
```

where value_1-n are 32-bit floats that form a vector corresponding to the token.

### Running

Either create a build using `cargo build --release` or use `cargo run` on your dataset.

```
nearest-neighbors [-a <algorithm>] -q <query>
```

> Use the `--path` flag or set the `DATASET_PATH` env var to specify the path of your dataset.

### Benchmarking

Running `cargo bench` will generate a report at `./target/criterion/report/index.html`.

> The benchmarking script expects an env variable `DATASET_PATH` pointing to your dataset.

## Author

- Kunal Kundu - [@tinfoil-knight](https://github.com/tinfoil-knight)

## License

Distributed under the MIT License. See [LICENSE](./LICENSE) for more information.

## References

- [K-d Trees - Computerphile](https://www.youtube.com/watch?v=BK5x7IUTIyU)
- [VP trees: A data structure for finding stuff fast](https://stevehanov.ca/blog/index.php?id=130)
- [Similarity search 101 - Part 2 (Fast retrieval with vp-trees)](https://everyhue.me/posts/similarity-search-101-with-vantage-point-trees/)
- [Introduction to Locality-Sensitive Hashing](https://tylerneylon.com/a/lsh1/)

## Datasets

- Jeffrey Pennington, Richard Socher, and Christopher D. Manning. 2014. [GloVe: Global Vectors for Word Representation](https://nlp.stanford.edu/pubs/glove.pdf).
  - https://github.com/stanfordnlp/GloVe
