# Nativo NFT - Marketplace


## Prepare the ENV
### Last Dev
` export CONTRACT="dev-1667958216740-89040802063606" `
### Sub account
` export CONTRACT="events.nativo-market.testnet"         `
` export CONTRACT="v4.nativo-market.testnet"         `

### Compile and make a contract devdeploy run:
` ./build_develop.sh  `
### Compile and make a contract deploy run:
` ./build.sh  `
### Compile and  make a contract migration run:
` ./migrate.sh `


### Initialize the market contract
`near call $CONTRACT new '{"owner_id":"dokxo.testnet"}'  --accountId dokxo.testnet`
## Contract state variables
### Pay the storage before to list a token
`near call  $CONTRACT storage_deposit  '{"account_id":"dokxo.testnet"}' --accountId dokxo.testnet`

### Get the contract owner address
`near view $CONTRACT get_owner_account`
### Get the contract treasury address
`near view $CONTRACT get_treasury`
### Get the fee for buy and sell 
`near view $CONTRACT  get_mint_fee`
### Get the ntv multiplier for buy and sell 
`near view $CONTRACT  get_ntv_multiplier`


### Set the contract owner address
`near call error.nativo-market.testnet set_owner_account '{"new_account":"dokxo_testnet"}' --accountId dokxo.testnet`
### Set the contract treasury address
`near call $CONTRACT set_treasury '{"new_account":"joehank.testnet"}' --accountId dokxo.testnet`
### Set the fee for buy and sell 
`near call $CONTRACT set_mint_fee '{"mint_fee":0.995}' --accountId dokxo.testnet`
### Set the ntv multiplier for buy and sell 
`near call $CONTRACT  set_ntv_multiplier '{"multiplier":4}' --accountId dokxo.testnet`
### Set a new contract in the whitelist
`near call $CONTRACT add_new_ext_contract '{"address_contract":"minterv2.nativo-minter.testnet","contract_name":"Nativo minter"}' --accountId dokxo.testnet `

## Create proposals in the DAO
## Make an update proposal to add a new feature
`sputnikdao proposal upgrade ./res/nativo_market_std.wasm error.nativo-market.testnet --daoAcc nativo-dao --accountId darkdokxo.testnet`

## Update the Market owner
`sputnikdao proposal call  $CONTRACT set_owner_account '{"new_account":"alexiaab.testnet"}' --daoAcc nativo-dao --accountId nativo-market.testnet`

## Update the Market treasury
`sputnikdao proposal call  $CONTRACT set_treasury '{"new_account":"dokxo.testnet"}' --daoAcc nativo-dao --accountId nativo-market.testnet`

## Market methods
### Get a sale by id
`near call $CONTRACT get_sale '{"nft_contract_token":"minterv2.nativo-minter.testnet.7"}' --accountId dokxo.testnet`

### Get the sales  by owner 
`near view $CONTRACT get_sales_by_owner_id '{"account_id":"joehank.testnet","from_index":"0","limit":25}'`

### Get the storage balance for the account
`near view $CONTRACT storage_balance_of  '{"account_id":"alexiaab.testnet"}'`
### Get the total sales for a contract address.
` near view $CONTRACT get_sales_by_nft_contract_id '{"nft_contract_id":"minterv2.nativo-minter.testnet","from_index":"0","limit":25}'`
### Update the price for a token_id
`near call $CONTRACT update_price '{"nft_contract_id":"mktstandard.testnet","token_id": "227","price":"10000000000000000000000"}' --account_id joehank.testnet --depositYocto 1`


### Offer for a token 
`near call $CONTRACT offer '{"account_id":"dokxo.testnet","nft_contract_id":"minterv2.nativo-minter.testnet","token_id":"93"}' --accountId darkdokxo.testnet --deposit 1 --gas=300000000000000`

### Remove the token from the market
`near call $CONTRACT remove_sale '{"nft_contract_id":"minterv2.nativo-minter.testnet","token_id": "93"}' --account_id dokxo_test.testnet --depositYocto 1  `
 

 

 ## Sale a NFT 
### get the total suply of Sales
`near view $CONTRACT get_supply_sales  --accountId dokxo.testnet`

### get the total Sales's by owner   
`near view $CONTRACT get_supply_by_owner_id '{"account_id":"dokxo.testnet"}' --accountId dokxo.testnet`

### get the  Sales by owner
`near view $CONTRACT get_sales_by_owner_id '{"account_id":"dokxo.testnet","from_index":"0","limit":10 }' --accountId dokxo.testnet`

### get the total  Sales's supply by contract  
`near view $CONTRACT get_supply_by_nft_contract_id '{"nft_contract_id":"minterv2.nativo-minter.testnet"}' --accountId dokxo.testnet`

### get the total  Sales's by contract  
`near view $CONTRACT get_sales_by_nft_contract_id '{"nft_contract_id":"minterv2.nativo-minter.testnet","from_index":"0","limit":10 }' --accountId dokxo.testnet`

### list as sales 
`near call minterv2.nativo-minter.testnet nft_approve '{"token_id":"93","account_id":"events.nativo-market.testnet","msg":"{\"market_type\":\"on_sale\",\"price\":\"7000000000000000000000\",\"title\":\"flames\",\"media\":\"bafybeib6hehfeyl5tmtj7w4uqwhtfhlyavmnkro5xdh4s224fiqlrykcay\",\"creator_id\":\"dokxo.testnet\"}"}' --accountId dokxo.testnet --deposit 0.1 --gas=300000000000000 `


## Offer for a token
###  Offer for a token in the gallery or the market
`near call $CONTRACT add_offer  '{"nft_contract_id":"minterv2.nativo-minter.testnet","token_id":"42","owner_id":"nativo-market.testnet"}' --accountId dokxo.testnet  --deposit 0.4 `

###  Delete the offer if you are the bidder
`near call $CONTRACT delete_offer '{"nft_contract_id":"minterv2.nativo-minter.testnet","token_id":"93"}' --accountId darkdokxo.testnet  --depositYocto 1`

### Get an offer for a token if exists
`near view $CONTRACT get_offer '{"nft_contract_id":"minterv2.nativo-minter.testnet","token_id":"93"}' --accountId dokxo.testnet`
### Get the total offers's by owner   
`near view $CONTRACT get_supply_offers_by_owner_id '{"account_id":"dokxo.testnet"}' --accountId dokxo.testnet`
### Get the total offers's by bidder
`near view $CONTRACT get_supply_offers_by_bidder_id '{"account_id":"darkdokxo.testnet"}' --accountId dokxo.testnet`
### Get the  offers by owner
`near view $CONTRACT get_offers_by_owner_id '{"account_id":"dokxo.testnet","from_index":"0","limit":10 }' --accountId dokxo.testnet `
### Get the  offers by bidder
`near view $CONTRACT get_offers_by_bidder_id '{"account_id":"dokxo_test.testnet","from_index":"0","limit":18 }' --accountId dokxo.testnet`

### get the total  offers's supply by contract  
`near view $CONTRACT get_supply_offers_by_nft_contract_id '{"nft_contract_id":"minterv2.nativo-minter.testnet"}' --accountId dokxo.testnet`

### get the total  offers's by contract  
`near view $CONTRACT get_offers_by_nft_contract_id '{"nft_contract_id":"minterv2.nativo-minter.testnet","from_index":"0","limit":10 }' --accountId dokxo.testnet`



### Set info for the profile  
`near call $CONTRACT add_new_profile '{"username": "dokxo.testnet","media": "Qmdx4EcG3mMKwLSktx9VyTB1ZueH1f4AKSKoSqr2s38FQQ","media_banner": "QmeyndES1Toq4ee8tTsHKkUL3pn7rprhfQNaL8XmeUMwYE","biography": "its dokxo description","social_media": "itsdokxo","_type": "edit"}' --accountId dokxo.testnet`


