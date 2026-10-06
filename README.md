# aleo-cli

[![crates.io](https://img.shields.io/crates/v/aleo-cli.svg)](https://crates.io/crates/aleo-cli)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue)](LICENSE-MIT)
[![CI](https://github.com/qiaopengjun5162/aleo-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/qiaopengjun5162/aleo-cli/actions)

CLI tool for the **Aleo blockchain** — balance checks, **private/public transfers** (fully working v0.4.0+), contract deployment, execution, local ZK proving, and on-chain transaction verification.

Built on the [`aleo-rust-sdk`](https://crates.io/crates/aleo-rust-sdk) v0.5.0.

## Installation

```bash
cargo install aleo-cli
```

## Commands

| Command | Description | Network |
|---------|-------------|---------|
| `generate` | Generate a new Aleo account (private key / view key / address) | offline |
| `query` | Query testnet state (block height, state root, program info) | HTTP |
| `balance` | Query public balance + scan private records | HTTP + RPC |
| `transfer` | Transfer credits — **`--mode private` fully working** (v0.4.0+) | full pipeline |
| `deploy` | Deploy a `.aleo` program to the chain | full pipeline |
| `exec` | Execute a program function (3 modes) | varies |
| `verify` | Verify a transaction (basic info or `--deep` ZK proof) | HTTP |

### Execution modes (`exec`)

- **Default**: prove + broadcast to network (full on-chain execution)
- **`--local`**: dry-run without ZK proof, millisecond response (credits.aleo only)
- **`--prove`**: generate complete ZK proof but do not broadcast

### Verification modes (`verify`)

- **Default**: fetch and display transaction type, status, owner
- **`--deep`**: fetch, deserialize, and locally verify the ZK proof

## Usage

Set your private key as environment variable or pass via `--private-key`:

```bash
export ALEO_PRIVATE_KEY="APrivateKey1..."
```

### Generate a new account

```bash
aleo-cli generate
```

### Query testnet state

```bash
aleo-cli query
```

### Check balance

```bash
aleo-cli balance
```

### Transfer credits

```bash
# Public transfer
aleo-cli transfer aleo1ss6e8... 1000000 --mode public

# Private transfer (v0.4.0+) — decrypts ciphertext records, fetches live Merkle paths
aleo-cli transfer aleo1ss6e8... 30000 --mode private
```

### Deploy a program

```bash
aleo-cli deploy /path/to/program.aleo program_name
```

### Execute locally (no proof, no broadcast)

```bash
aleo-cli exec credits.aleo transfer_public aleo1ss6e8... 1000u64 --local
```

### Execute with proof (no broadcast)

```bash
aleo-cli exec credits.aleo transfer_public aleo1ss6e8... 1000u64 --prove --base-fee 50000
```

### Execute and broadcast

```bash
aleo-cli exec test_cli_deploy.aleo hello 42u32 --base-fee 50000
```

### Verify a transaction

```bash
# Basic info
aleo-cli --node "https://api.provable.com/v2/testnet" verify at136grnr...

# Deep ZK proof verification
aleo-cli --node "https://api.provable.com/v2/testnet" verify at136grnr... --deep
```

## Private Transfer (How It Works)

`transfer_private` in v0.4.0+ performs these steps automatically:

1. **Scan for records** — calls `scan_recent(5000)` to find unspent ciphertext records
2. **Decrypt** — uses the ViewKey to decrypt each `Ciphertext` into a `Record`
3. **Parse values** — converts the `Record` into `Value::Record` (skips the `Value::from_str` issue with `record1` format)
4. **ProvableQuery** — fetches real Merkle state paths from `api.provable.com/v2/testnet/statePath/{commitment}` (not dummy queries)
5. **InclusionVersion::V1** — uses the correct SNARK verifying key (V1, matching ConsensusVersion::V14)
6. **Broadcast** — submits and confirms on-chain

**Verification:** `aleo-cli verify <tx_id> --deep` fetches the transaction from the network, deserializes the execution, and re-verifies the ZK proof locally. This is a true cryptographic verification — not a metadata lookup.

## Configuration

- `--private-key` / `ALEO_PRIVATE_KEY` — account private key
- `--node` — Aleo node URL (default: `https://api.provable.com/v2/testnet`)
- `--base-fee` — base fee in microcredits (for exec/deploy)
- `--priority-fee` — priority fee in microcredits (optional)

## Pre-commit Quality Gates

This project uses pre-commit to enforce code quality on every commit:

```bash
# Install hooks (one-time after clone)
pre-commit install --hook-type pre-commit --hook-type commit-msg

# Or run all checks manually
pre-commit run --all-files
```

The following 9 checks run automatically:

| # | Hook | What it checks |
|---|------|---------------|
| 1 | fix-byte-order-marker | BOM encoding |
| 2 | check-case-conflict | Case-sensitive filename conflicts |
| 3 | check-merge-conflict | Unresolved merge markers |
| 4 | check-yaml | YAML syntax validity |
| 5 | end-of-file-fixer | Files end with newline |
| 6 | mixed-line-ending | Consistent line endings |
| 7 | trailing-whitespace | No trailing whitespace |
| 8 | cargo fmt | Rust formatting |
| 9 | cargo check | Compilation |
| 10 | cargo clippy | Lint (`-- -D warnings`) |
| 11 | typos | Spelling errors |

> **Note:** All local hooks use `language: system` (not `language: rust`). This was a bug fix in v0.4.0 — earlier versions used `language: rust` which caused hooks to silently skip.

## Project Layout

```
src/
├── main.rs                # clap CLI entry point
└── commands/
    ├── mod.rs
    ├── balance.rs         # AleoHttpClient::fetch_mapping_value + records scan
    ├── generate.rs        # AleoAccount::new_random
    ├── query.rs           # Network state queries
    ├── transfer.rs        # transfer_public / transfer_private (with decryption + ProvableQuery)
    ├── deploy.rs          # AleoClient::deploy_program (8-step pipeline)
    ├── exec.rs            # 3-mode execution
    └── verify.rs          # Transaction verification
```

## References

- [aleo-rust-sdk](https://crates.io/crates/aleo-rust-sdk) — Rust SDK this CLI depends on
- [Aleo Developer Docs](https://developer.aleo.org)
- [Aleo Testnet Explorer](https://explorer.provable.com/testnet)
- [Provable API v2 docs](https://docs.explorer.provable.com/docs/api/v2/intro)

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
