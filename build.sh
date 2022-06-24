#!/bin/bash
set -e
cd "`dirname $0`"
source flags.sh
cargo build --all --target wasm32-unknown-unknown --release

if [ ! -d res/ ];
then
mkdir res
fi

cp target/wasm32-unknown-unknown/release/nativo_market_std.wasm ./res/

echo "¿Quieres desplegar el contrato de market?"
select yn in "Si" "No"; do
    case $yn in
        Si ) near deploy $CONTRACT --wasmFile res/nativo_market_std.wasm; break;;
        No ) exit;;
    esac
done