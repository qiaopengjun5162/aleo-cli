# aleo-cli

[![crates.io](https://img.shields.io/crates/v/aleo-cli.svg)](https://crates.io/crates/aleo-cli)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue)](LICENSE-MIT)
[![CI](https://github.com/qiaopengjun5162/aleo-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/qiaopengjun5162/aleo-cli/actions)

CLI tool for the **Aleo blockchain** — balance checks, private/public transfers, contract deployment, execution, local ZK proving, and on-chain transaction verification.

Built on the [`aleo-rust-sdk`](https://crates.io/crates/aleo-rust-sdk).

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
| `transfer` | Transfer credits (public or private mode) | full pipeline |
| `deploy` | Deploy a `.aleo` program to the chain | full pipeline |
| `exec` | Execute a program function (3 modes: default / `--local` / `--prove`) | varies |
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
aleo-cli transfer aleo1ss6e8... 1000000 --mode public
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
aleo-cli --node "https://api.explorer.provable.com/v2/testnet" verify at1z9elr8...

# Deep ZK proof verification
aleo-cli --node "https://api.explorer.provable.com/v2/testnet" verify at1z9elr8... --deep
```

## Configuration

- `--private-key` / `ALEO_PRIVATE_KEY` — account private key
- `--node` — Aleo node URL (default: `https://api.explorer.provable.com/v2/testnet`)
- `--base-fee` — base fee in microcredits (for exec/deploy)
- `--priority-fee` — priority fee in microcredits (optional)

## Project Layout

```
src/
├── main.rs                # clap CLI entry point
└── commands/
    ├── mod.rs
    ├── balance.rs         # AleoHttpClient::fetch_mapping_value + records scan
    ├── generate.rs        # AleoAccount::new_random
    ├── query.rs           # Network state queries
    ├── transfer.rs        # transfer_public / transfer_private
    ├── deploy.rs          # AleoClient::deploy_program (8-step pipeline)
    ├── exec.rs            # 3-mode execution
    └── verify.rs          # Transaction verification
```

## References

- [aleo-rust-sdk](https://crates.io/crates/aleo-rust-sdk) — Rust SDK this CLI depends on
- [Aleo Developer Docs](https://developer.aleo.org)
- [Aleo Testnet Explorer](https://explorer.provable.com/testnet)

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
