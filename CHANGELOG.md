# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2026-10-05

### Features

- `generate` — Generate new Aleo account (private key / view key / address)
- `query` — Query testnet state (block height, state root, program info)
- `balance` — Query public balance + scan private records
- `transfer` — Public/private transfer with dry-run + prove + broadcast
- `deploy` — Deploy .aleo program to chain (8-step pipeline)
- `exec` — 3-mode program execution:
  - Default: prove + broadcast
  - `--local`: dry-run without proof (milliseconds)
  - `--prove`: generate proof without broadcast
- `verify` — 2-mode transaction verification:
  - Default: fetch and display tx info
  - `--deep`: local ZK proof verification

### Bug Fixes

- `tx_id` quote trimming from broadcast response
- Custom program execution (fetch from network, not local only)
- Edition reset for new program deployment
- Deploy proof verification (empty engine to avoid program ID conflicts)

### Notes

- Built on aleo-rust-sdk v0.4.0
- All commands tested against real Aleo Testnet
