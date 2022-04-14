
### create a subaccount
`near create-account v1.nativo-market.testnet --masterAccount nativo-market.testnet`
### delete a subaccount
`near delete v1.nativo-market.testnet nativo-market.testnet`

### Compile,build and deploy the market contract 
`./build.sh`
### Set the market contract global
`export CONTRACT="v1.nativo-market.testnet" `
### initialize the market contract
`near call $CONTRACT new '{"owner_id":"dokxo.testnet"}'  --accountId dokxo.testnet`
### to pay the storage before to list a token
`near call  $CONTRACT storage_deposit  '{"account_id":"dokxo.testnet"}' --accountId dokxo.testnet`

### get the contract owner address
`near view $CONTRACT get_owner_account`

### set the contract owner address
`near call $CONTRACT set_owner_account '{"new_account":"nativo-dao.sputnikv2.testnet"}' --accountId nativo-market.testnet`

### get the contract treasury address
`near view $CONTRACT get_treasury`

### set the contract treasury address
`near call $CONTRACT set_treasury '{"new_account":"joehank.testnet"}' --accountId dokxo.testnet`

## Crear una nueva propuesta de actualizacion desde la DAO(testeado)
`sputnikdao proposal upgrade ./res/nft_simple.wasm $CONTRACT --daoAcc nativo-dao --accountId dokxo.testnet`

## Crear una nueva propuesta para la actualizacion del dueño del market desde la DAO(testeado)
`sputnikdao proposal call  $CONTRACT set_owner_account '{"new_account":"dokxo.testnet"}' --daoAcc nativo-dao --accountId nativo-market.testnet`

## Crear una nueva propuesta para la actualizacion del tesorero del market desde la DAO(testeado)
`sputnikdao proposal call  $CONTRACT set_treasury '{"new_account":"dokxo.testnet"}' --daoAcc nativo-dao --accountId nativo-market.testnet`


### get the sales  by owner 
`near view $CONTRACT get_sales_by_owner_id '{"account_id":"customnativo.testnet","from_index":"0","limit":25}'`

### get the storage balance for the account
`near view $CONTRACT storage_balance_of  '{"account_id":"alexiaab.testnet"}'`
### get the total sales for a contract address.
`near view $CONTRACT get_supply_by_nft_contract_id '{"nft_contract_id":"hardtest.nativo-minter.testnet"}'`

### update the price for a token_id
`near call stdmarket.testnet update_price '{"nft_contract_id":"mktstandard.testnet","token_id": "227","price":"10000000000000000000000"}' --account_id joehank.testnet --depositYocto 1`

### make a offer for a token 
`near call $CONTRACT offer '{"account_id":"dokxo.testnet","nft_contract_id":"dev-1649875934003-18318891633598","token_id":"9"}' --accountId dokxo.testnet --deposit 1 --gas=300000000000000`

### make a deposit for storage payment 
`near call $ID storage_deposit '{"account_id":"dokxo.testnet"}' --accountId dokxo.testnet —deposit 0.1`






near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/Nativo_market_std.wasm

`near call $CONTRACT new '{"owner_id":"dokxo.testnet"}'  --accountId dokxo.testnet`




near call hardtest.nativo-minter.testnet nft_approve '{"token_id":"9","account_id":"dev-1649875934003-18318891633598","msg":"{\"sale_conditions\":\"10000000000000000000000000\"}"}' --accountId dokxo.testnet --deposit 1

