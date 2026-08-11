# Solana Banking App

A decentralized banking application built on the Solana blockchain using Rust, Anchor Framework, and JavaScript.

## Overview

This project is a blockchain-based banking application developed as part of a software development internship. The application demonstrates how a Solana program can manage basic banking operations through on-chain instructions.

The project consists of a Rust-based Solana program and a JavaScript client used to communicate with the program.

## Features

- Initialize a user banking account
- Deposit funds
- Withdraw funds
- Store account state on Solana
- Interact with the program through JavaScript
- Connect to the Solana Devnet
- PDA-based account management

## Technologies

- Rust
- Anchor Framework
- Solana
- JavaScript
- Node.js
- `@coral-xyz/anchor`
- `@solana/web3.js`

## Project Structure

```text
solana-banking-app/
├── programs/
│   └── penne_bank_app/
│       └── src/
│           ├── instructions/
│           │   ├── initialize.rs
│           │   ├── deposit.rs
│           │   └── withdraw.rs
│           ├── constants.rs
│           ├── error.rs
│           ├── instructions.rs
│           ├── lib.rs
│           └── state.rs
│
├── app/
│   ├── src/
│   │   └── index.js
│   ├── package.json
│   └── package-lock.json
│
├── Anchor.toml
├── Cargo.toml
└── README.md
