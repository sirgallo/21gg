# 21gg - program


## overview

`21gg` utilizes solana native rust. 

Why?

  - I didn't like the restrictions and confusing boilerplate provided by the anchor
  - I wanted to get to know the Solana runtime and Rust better


## prereq

Follow direction to install the [Solana Cli](https://docs.solanalabs.com/cli/install)


## build 

```bash
sudo cargo build-bpf --manifest-path programs/athn/Cargo.toml
```


**Note**

If failure on build due to rustc version mismatch, run this before `cargo build-bpf`:
```bash
solana-install update
```