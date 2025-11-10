# Anchor Escrow

A professional, secure escrow program for the Solana blockchain built with the Anchor framework. Anchor Escrow enables a maker to lock tokens into a PDA-backed vault and a taker to fulfill the trade — or the maker to refund — using secure CPI token transfers and PDA signing.

## Overview

`anchor_escrow` is an on-chain Solana program that implements a token escrow flow using Program Derived Addresses (PDAs) for deterministic, program-owned accounts. It provides a minimal, secure set of instructions to create an escrow (maker deposits tokens), take the escrow (taker completes the trade), and refund (maker recovers funds when the trade is not completed).

Key capabilities:
- Create an escrow with a vault PDA to hold deposited tokens
- Maker deposits tokens into the vault via CPI to the token program
- Taker can fulfill the trade; atomic token transfers happen with PDA signing
- Maker can refund and close the escrow, returning funds and closing PDAs

## Features

- PDA-based security: deterministic vault and state PDAs for safe account ownership and signing
- Token-safe CPIs: uses the SPL token program (via Anchor) for all token transfers and account closures
- Rent-exempt account initialization: escrow state and vault are sized and funded to be rent-exempt
- Owner-validated actions: account constraints and seeds ensure only authorized actors perform critical operations
- Tests & benchmarking: includes both Anchor TypeScript integration tests and Mollusk instruction-level tests/benches

## Quick contract summary

- Instructions: `make` (create escrow & deposit), `take` (fulfill escrow), `refund` (maker reclaims and closes)
- Primary accounts: `Escrow` state account (PDA), `Vault` token account (PDA), maker/taker token accounts

See the program entry and instruction implementations in `programs/anchor_escrow/src/`.

## Installation

Prerequisites
- Rust (stable toolchain)
- Solana CLI
- Anchor CLI (v0.31.x compatible)
- Node.js (v16+) and Yarn or npm
- Mollusk (optional, for SBF instruction testing and benchmarking)

Clone the repository:

```sh
git clone https://github.com/ANISH-SR/anchor_escrow.git
cd anchor_escrow
```

Build the on-chain program:

```sh
anchor build
```

Install JS dependencies (for integration tests and scripts):

```sh
yarn install
# or: npm install
```

## Configuration

- Anchor config is in `Anchor.toml`. Update the `[provider]` section to set your cluster (`localnet`, `devnet`, or `mainnet-beta`) and wallet path.
- Program IDs are declared in the program crate and `Anchor.toml`. Make sure they match when deploying.

## Usage

Build

```sh
anchor build
```

Run TypeScript/Anchor integration tests

```sh
anchor test
```

Run Rust instruction-level tests & benches (Mollusk)

```sh
cargo test-sbf
```

Local development (quick start)

1. Start a local validator:

```sh
solana-test-validator
```

2. Build and deploy to localnet:

```sh
anchor build
anchor deploy
```

Or use the migration script in `migrations/deploy.ts` to deploy from a scripted step.

## Program Instructions (high level)

1. Make (create escrow and deposit)
   - Creates an `Escrow` state PDA and a program-owned `Vault` token account (PDA).
   - Transfers the maker's deposit into the vault via token CPI.

2. Take (fulfill the escrow)
   - Taker provides the expected counterparty tokens.
   - Program atomically transfers tokens from vault to taker and from taker to maker with PDA signing, then closes the vault and escrow state.

3. Refund (maker reclaims funds)
   - If escrow not taken, maker may refund: transfer tokens from vault back to maker and close the PDA accounts.

For full account details and exact argument signatures, see the instruction handlers in `programs/anchor_escrow/src/instructions/` and the program entry in `programs/anchor_escrow/src/lib.rs`.

## Project structure

Top-level layout:

```
anchor_escrow/
├─ programs/
│  └─ anchor_escrow/            # Rust on-chain program
│     ├─ src/
│     │  ├─ lib.rs               # Program declaration & exported handlers
│     │  ├─ instructions/        # make.rs, take.rs, refund.rs
│     │  ├─ state/               # Escrow state account definitions
│     │  ├─ constants.rs
│     │  └─ error.rs
│     └─ tests/                  # Mollusk instruction tests & benches
├─ migrations/                   # Deployment scripts (TypeScript)
├─ tests/                        # Anchor TypeScript integration tests
├─ Anchor.toml                   # Anchor configuration
├─ Cargo.toml                    # Workspace and dependency configuration
└─ package.json                  # JS dependencies & scripts
```

## Testing

- Integration (Anchor): `anchor test` runs the TypeScript tests under `tests/` using the Anchor provider.
- Instruction-level (Mollusk): `cargo test-sbf` runs SBF tests and compute-unit benches found under `programs/anchor_escrow/tests/`.

Benchmark outputs (compute unit results) are saved in the program benches folder when the Mollusk bencher is run.

## Security

- All critical account relationships are validated via Anchor account constraints (e.g., `has_one`, `seeds`, `bump`).
- Token transfers and account closures use the SPL token program CPIs, and PDA signing is used for vault operations.
- Use rent-exempt funding when creating state and vault accounts.

## Contributing

- Contributions are welcome. Please open issues for bugs or improvement proposals.
- Workflow:
  1. Fork and branch from `main`.
  2. Implement changes with tests (Anchor and/or Mollusk where applicable).
  3. Run `anchor build` and `cargo test-sbf` before opening a PR.

## Dependencies

- Anchor framework (program crate) — see `programs/anchor_escrow/Cargo.toml` for exact versions (typically Anchor 0.31.x compatibility).
- Node-side: `@coral-xyz/anchor` (TypeScript client) and test tooling.
- Mollusk — used for instruction-level testing / benchmarking.

## Acknowledgments

- Built with the Anchor framework and the Solana toolchain.
- Mollusk for low-level instruction testing and compute-unit benchmarking.

---

If you want edits (shorter/longer, include a program ID, or link README badges), tell me which details to change and I will update the file.
