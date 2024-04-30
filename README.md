# {{crate_name}}
## Install

```bash
# install pre-commit
pipx install pre-commit

# install Cargo deny
cargo install --locked cargo-deny

# install typos-cli
cargo install typos-cli

# install git-cliff
cargo install git-cliff

# install nextest
cargo install --locked  cargo-nextest
```

## Initial

### initial pre-commit

```bash
pre-commit install
```

## Run
```bash
cargo run
```

## Test
```bash
cargo nextest run
```

## Build
```bash
cargo build --release
```
