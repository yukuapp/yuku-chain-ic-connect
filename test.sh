#!/usr/bin/env bash

dfx stop
trap 'dfx stop' EXIT

dfx start --background --clean

# deploy
cargo test print_candid -- --nocapture
dfx deploy yuku_chain_ic_connect

echo ""
read -s -n1 -p "Press any key to end..."
