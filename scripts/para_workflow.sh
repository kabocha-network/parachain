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
CHAIN_TYPE
RELAY_CHAIN_SPEC
CHAIN_NAME
PROTOCOL_ID
SPEC_NAME
RAW_SPEC_NAME
PARA_ID
PORT
WS_PORT
RPC_PORT
RELAY_PORT
RELAY_WS_PORT

#ENV FILE
AURA_KEY



#If $CHAINTYPE = parachain then;

#STEP1
 ./target/release/$CHAIN_NAME build-spec --chain ./specs/$SPEC_NAME --raw > ./specs/$RAWSPEC_NAME.json

#STEP2
# Create genesis head
 ./target/release/$CHAIN_NAME export-genesis-state  --chain ./specs/$RAWSPEC_NAME > ./genesiswasm/$PROTOCOL_ID-$PARA_ID-genesis

#STEP3
# Create genesis wasm
./target/release/$CHAIN_NAME export-genesis-wasm  --chain ./specs/$RAWSPEC_NAME > ./genesiswasm/$PROTOCOL_ID-$PARA_ID-wasm


# Launch collator node as a systemd

PORT
WS_PORT
RPC_PORT
RELAY_PORT
RELAY_WS_PORT

#STEP 4
./target/release/$CHAIN_NAME --collator --force-authoring --chain ./specs/$RAWSPEC_NAME --base-path /home/$USERNAME/$CHAINTYPE/SOUP1-2007 --port $PORT --ws-port $WS_PORT --rpc-port $RPC_PORT --rpc-methods=Unsafe -- --execution wasm --chain ./specs/$RELAY_CHAIN_SPEC --port $RELAY_PORT --ws-port $RELAY_WS_PORT

# Submit aura key (curl RPC call) check AURA in an ENV file


#


# Submit parasudoinitialize