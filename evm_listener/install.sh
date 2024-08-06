#!/bin/bash

# Set the contract file path and the output directory
CONTRACT_FILE="solidity/contracts/Mailbox.sol"
OUTPUT_DIR="build"

# Ensure the output directory exists
mkdir -p $OUTPUT_DIR

# Install npm dependencies with --legacy-peer-deps (if not already installed)
npm install --legacy-peer-deps

# Install Foundry dependencies (if not already installed)
forge install

# Compile the contract and output ABI and binary
solc --abi --bin --include-path node_modules --include-path lib --base-path . -o $OUTPUT_DIR $CONTRACT_FILE

echo "Compilation complete. ABI and binary files are in $OUTPUT_DIR"

