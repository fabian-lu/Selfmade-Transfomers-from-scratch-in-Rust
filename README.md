# Transformers from scratch in Rust

This is a learning project. I'm building a small transformer language model and, later, a small tabular foundation model in the style of TabPFN, completely by hand in Rust. I don't use any machine learning libraries, so the matrix code, the backpropagation and the training loop are all written from scratch.

I want to better understand Tabular Foundation models, and next to it learn Rust. I started this project with no Rust experience at all, so the code grows together with what I learn. Expect it to be rough in places and to change a lot.

## The plan

1. A matrix type and a random number generator.
2. A small neural network with backpropagation written by hand, trained on XOR and a spiral dataset.
3. An autograd engine, so that gradients are computed automatically.
4. A character-level language model trained on Shakespeare, first as a bigram model and then as a small MLP.
5. Attention and transformer blocks, put together into a mini-GPT.
6. A mini tabular foundation model in the style of TabPFN, trained on synthetic datasets.

After that, I'll pick extras by curiosity, like a BPE tokenizer, RoPE, a KV cache, or porting the model to a Rust ML framework such as Candle or Burn to compare the two.

## Running it

You need Rust, which you can install with [rustup](https://rustup.rs). Then run:

```
cargo run
```

The files in `src/bin/` are small playground programs where I try out Rust features. You run one with `cargo run --bin <name>`, for example `cargo run --bin functional_programming_stuff`.

## What I'm learning from

- The Rust Book: https://doc.rust-lang.org/book/
- Andrej Karpathy, "Neural Networks: Zero to Hero": https://karpathy.ai/zero-to-hero.html
- Vaswani et al. 2017, "Attention Is All You Need": https://arxiv.org/abs/1706.03762
- Hollmann et al. 2023, "TabPFN: A Transformer That Solves Small Tabular Classification Problems in a Second": https://arxiv.org/abs/2207.01848
