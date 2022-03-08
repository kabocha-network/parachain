#!/bin/bash/env


#VARIABLES
export CHAIN_TYPE=parachain
export RELAY_CHAIN_SPEC="../specs/pop-art-3-val.json"
export CHAIN_NAME="parachain-collator"
export PROTOCOL_ID=soupcan
export SPEC_NAME="kabocha-soupcan-dev-plain-2068-1.json"
export RAWSPEC_NAME="2068-raw.json"

export PARA_ID=2068

export GENESIS_HEAD='../genesiswasm/'"$PROTOCOL_ID"'-'"$PARA_ID"'-genesis'
export GENESIS_WASM='../genesiswasm/'"$PROTOCOL_ID"'-'"$PARA_ID"'-wasm'
export BASE_PATH="/home/$USERNAME/$CHAIN_TYPE"


export PORT=30333
export WS_PORT=9944
export RPC_PORT=9933
export RELAY_PORT=40333
export RELAY_WS_PORT=8844
export USERNAME=decentration