# deployment


## prereq

Follow direction to install the [Solana Cli](https://docs.solanalabs.com/cli/install)


## configure

Set the Solana CLI to point to desired deployment endpoint (for local, `endpoint` is localhost)
```bash
solana config set --url <endpoint>
```

## build 

From the root of the project (since this is a monorepo):
```bash
sudo cargo build-bpf --manifest-path program/Cargo.toml
```

**Note**

If failure on build due to rustc version mismatch, run this before `cargo build-bpf`:
```bash
solana-install update
```

## deploy

The compiled bytecode is placed in the target directory, from where it can then be deployed:
```bash
solana program deploy ./target/deploy/<deployment>.so
```