#!/bin/bash

source "./config.sh"


#STEP2   - Create genesis head
../target/release/$CHAIN_NAME export-genesis-state  --chain ../specs/$RAWSPEC_NAME > $GENESIS_HEAD

echo "$GENESIS_HEAD created"

