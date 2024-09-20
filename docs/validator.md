# validator

### local solana validator and cli


## install

Pull latest release from `Solana` releases:
```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

For local purposes there should not be a reason to use any other release than `stable`, but `beta` and `edge` are both available as options, or specific release versions if needed.

`PATH` should be automatically updated, but may prompt to manually update `PATH` variables.

To check the installed version of `Solana` CLI:
```bash
solana --version
```


## update

To update the current local `Solana` validator and CLI to latest, run:
```bash
solana-install update
```


## commands

`


**NOTE**

For additional help on the command line:
```bash
solana --help
```