#!/bin/bash
set -e
cd "`dirname $0`"
source flags.sh
cargo build --all --target wasm32-unknown-unknown --release

if [ ! -d res/ ];
then
mkdir res
fi

cp target/wasm32-unknown-unknown/release/Nativo_market_std.wasm ./res/

echo "¿Quieres desplegar el contrato de market?"
select yn in "Si" "No"; do
    case $yn in
        Si ) near deploy v1.nativo-market.testnet --wasmFile res/Nativo_market_std.wasm; break;;
        No ) exit;;
    esac
done