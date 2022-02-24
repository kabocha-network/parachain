#!/bin/bash

# Create raw file
# Example of workflow 
# ./target/release/parachain-collator build-spec --chain ./specs/kabocha-local-plain2.json --raw > ./specs/kabocha-soupcan-raw.json

# Create genesis head
# ./target/release/parachain-collator export-genesis-state  --chain ./specs/kabocha-soupcan-raw.json > ./genesiswasm/soupcan-2001-genesis

# Create genesis wasm
# ./target/release/parachain-collator export-genesis-wasm   --chain ./specs/kabocha-soupcan-raw.json > ./genesiswasm/soupcan-2001-wasm


# Submit parasudoinitialize

#PSEUDOCODE (IN DEVELOPMENT)

#VARIABLES

source "./config.sh"
# export CHAIN_TYPE="./config.sh/$CHAIN_TYPE"
# export RELAY_CHAIN_SPEC="./config.sh/$RELAY_CHAIN_SPEC"
# export CHAIN_NAME="./config.sh/$CHAIN_NAME"
# export PROTOCOL_ID="./config.sh/$PROTOCOL_ID"
# export SPEC_NAME="./config.sh/$SPEC_NAME"
# export PARA_ID=".config.sh/$PARA_ID"
# export RAWSPEC_NAME=".config.sh/$RAWSPEC_NAME"
# export PORT=".config.sh/$PORT"
# export WS_PORT=".config.sh/$WS_PORT"
# export RPC_PORT=".config.sh/$RPC_PORT"
# export RELAY_PORT=".config.sh/$RELAY_PORT"
# export RELAY_WS_PORT=".config.sh/$RELAY_WS_PORT"
# export USERNAME=".config.sh/$USERNAME"
# export GENESIS_HEAD=".config.sh/$GENESIS_HEAD"
# export GENESIS_WASM=".config.sh/$GENESIS_WASM"
# export BASE_PATH=".config.sh/$BASE_PATH"

#ENV FILE
#AURA_KEY



#If $CHAINTYPE = parachain then;


#STEP 0  - build chain spec

../target/release/$CHAIN_NAME build-spec --chain dev > ../specs/$SPEC_NAME

echo "$SPEC_NAME created"

