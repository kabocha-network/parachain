#!/bin/bash



source "./config.sh"



#STEP3    - Create genesis wasm
../target/release/$CHAIN_NAME export-genesis-wasm  --chain ../specs/$RAWSPEC_NAME > $GENESIS_WASM

echo "$GENESIS_WASM created"


