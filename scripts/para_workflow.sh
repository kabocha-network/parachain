# Create raw file
# Example of workflow 
# ./target/release/parachain-collator build-spec --chain ./specs/kabocha-local-plain2.json --raw > ./specs/kabocha-soupcan-raw.json

# Create genesis head
# ./target/release/parachain-collator export-genesis-state  --chain ./specs/kabocha-soupcan-raw.json > ./genesiswasm/soupcan-2001-genesis

# Create genesis wasm
# ./target/release/parachain-collator export-genesis-wasm   --chain ./specs/kabocha-soupcan-raw.json > ./genesiswasm/soupcan-2001-wasm


# Submit parasudoinitialize

CHAINNAME
PROTOCOLID
SPECNAME
RAWSPECNAME

 ./target/release/$CHAINNAME build-spec --chain ./specs/$SPECNAME --raw > ./specs/$RAWSPECNAME.json

# Create genesis head
 ./target/release/$CHAINNAME export-genesis-state  --chain ./specs/$RAWSPECNAME > ./genesiswasm/$PROTOCOLID-$PARAID-genesis

# Create genesis wasm
./target/release/$CHAINNAME export-genesis-wasm  --chain ./specs/$RAWSPECNAME > ./genesiswasm/$PROTOCOLID-$PARAID-wasm


# Submit parasudoinitialize