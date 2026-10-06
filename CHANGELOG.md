# Changelog

All notable changes to this project will be documented in this file.

## [0.4.0] - 2026-10-06

### Features

- `transfer_private` — fully working private transfer with real-time Merkle path fetching and correct inclusion proving (InclusionVersion::V1)
- Uses `scan_recent(5_000)` for faster record scanning

### Bug Fixes

- `transfer_private` no longer panics on `verify_batch` — root cause was `InclusionVersion::V0` vs V1 mismatch with `ConsensusVersion::V14`

### Notes

- Built on aleo-rust-sdk v0.5.0
- `transfer_private` tested against real Aleo Testnet ✅

## [0.2.0] - 2026-10-06

### Features

- `record` — inspect private records with optional program filter (`--program`), spent display (`--include-spent`), and skip-scan mode (`--no-refresh`)

### Improvements

- Updated README with record command reference

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
