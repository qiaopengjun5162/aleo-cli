# Changelog

All notable changes to this project will be documented in this file.

## [0.6.0](https://github.com/qiaopengjun5162/aleo-cli/compare/aleo-cli-v0.5.0...aleo-cli-v0.6.0) (2026-10-10)


### Features

* add PR template, PR-Agent config, Release Please, pre-commit hooks, and stablecoin command ([eb0d456](https://github.com/qiaopengjun5162/aleo-cli/commit/eb0d45683942774688be5f3e23b3566f6bdf6687))
* add record command — inspect private records ([a183a7b](https://github.com/qiaopengjun5162/aleo-cli/commit/a183a7b1f9ef0477780a55fad9f2137dec1f395a))
* **cli:** add --local and --prove modes + verify command ([57aeb22](https://github.com/qiaopengjun5162/aleo-cli/commit/57aeb22f908470c6113106fa2acee850c2734224))
* **cli:** add deploy + exec commands — full coverage of AleoClient API ([57d8f28](https://github.com/qiaopengjun5162/aleo-cli/commit/57d8f28bd370799d83bc80051e3bf1ca4c4efd10))
* **cli:** add verify --deep for local ZK proof verification ([ec80570](https://github.com/qiaopengjun5162/aleo-cli/commit/ec8057077531fd0016e57a2e3715c7927083ce94))
* 完整的 transfer_private (records/all + decrypt) + balance 私密记录明细 ([23270b7](https://github.com/qiaopengjun5162/aleo-cli/commit/23270b75d703c73af44de6cf2d973b58eb5bd98c))


### Bug Fixes

* **exec:** skip dry-run for custom programs — credits.aleo-only dry-run, custom programs fetched from network ([50fdf28](https://github.com/qiaopengjun5162/aleo-cli/commit/50fdf28458c98b01d49771bbb0d05089da2e9d96))
* record command - use scan_recent(5K) instead of scan(100K) ([569fc0c](https://github.com/qiaopengjun5162/aleo-cli/commit/569fc0c8d3925465f89bacc47fc61588bbad541b))
* set release-please draft to false and fix pr-agent entryPoint ([1c67d1d](https://github.com/qiaopengjun5162/aleo-cli/commit/1c67d1d3c740b19ac4bc4f597169ee615bec96e8))
* switch aleo-rust-sdk from path to crates.io v0.5.1 for CI compatibility ([7f46169](https://github.com/qiaopengjun5162/aleo-cli/commit/7f461692a36fe6099189539bf74320574cf095e9))
* update pr-agent workflow and pre-commit config ([c584e1c](https://github.com/qiaopengjun5162/aleo-cli/commit/c584e1c29f91ba50f8334e36471445ea5437b595))

## [0.5.0](https://github.com/qiaopengjun5162/aleo-cli/compare/aleo-cli-v0.4.0...aleo-cli-v0.5.0) (2026-10-09)


### Features

* add PR template, PR-Agent config, Release Please, pre-commit hooks, and stablecoin command ([eb0d456](https://github.com/qiaopengjun5162/aleo-cli/commit/eb0d45683942774688be5f3e23b3566f6bdf6687))
* add record command — inspect private records ([a183a7b](https://github.com/qiaopengjun5162/aleo-cli/commit/a183a7b1f9ef0477780a55fad9f2137dec1f395a))
* **cli:** add --local and --prove modes + verify command ([57aeb22](https://github.com/qiaopengjun5162/aleo-cli/commit/57aeb22f908470c6113106fa2acee850c2734224))
* **cli:** add deploy + exec commands — full coverage of AleoClient API ([57d8f28](https://github.com/qiaopengjun5162/aleo-cli/commit/57d8f28bd370799d83bc80051e3bf1ca4c4efd10))
* **cli:** add verify --deep for local ZK proof verification ([ec80570](https://github.com/qiaopengjun5162/aleo-cli/commit/ec8057077531fd0016e57a2e3715c7927083ce94))
* 完整的 transfer_private (records/all + decrypt) + balance 私密记录明细 ([23270b7](https://github.com/qiaopengjun5162/aleo-cli/commit/23270b75d703c73af44de6cf2d973b58eb5bd98c))


### Bug Fixes

* **exec:** skip dry-run for custom programs — credits.aleo-only dry-run, custom programs fetched from network ([50fdf28](https://github.com/qiaopengjun5162/aleo-cli/commit/50fdf28458c98b01d49771bbb0d05089da2e9d96))
* record command - use scan_recent(5K) instead of scan(100K) ([569fc0c](https://github.com/qiaopengjun5162/aleo-cli/commit/569fc0c8d3925465f89bacc47fc61588bbad541b))
* switch aleo-rust-sdk from path to crates.io v0.5.1 for CI compatibility ([7f46169](https://github.com/qiaopengjun5162/aleo-cli/commit/7f461692a36fe6099189539bf74320574cf095e9))
* update pr-agent workflow and pre-commit config ([c584e1c](https://github.com/qiaopengjun5162/aleo-cli/commit/c584e1c29f91ba50f8334e36471445ea5437b595))

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
