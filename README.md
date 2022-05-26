
### create a subaccount
`near create-account v2.nativo-market.testnet --masterAccount nativo-market.testnet`
### delete a subaccount
`near delete v1.nativo-market.testnet nativo-market.testnet`

### Compile,build and deploy the market contract 
`./build.sh`
### Set the market contract global
`export CONTRACT="v2.nativo-market.testnet" `

export CONTRACT="dev-1652818492986-28657230673626"
 ### initialize the market contract
`near call $CONTRACT new '{"owner_id":"dokxo.testnet"}'  --accountId dokxo.testnet`
### to pay the storage before to list a token
`near call  $CONTRACT storage_deposit  '{"account_id":"dokxo.testnet"}' --accountId dokxo.testnet`

### get the contract owner address
`near view $CONTRACT get_owner_account`

### set the contract owner address
`near call $CONTRACT set_owner_account '{"new_account":"nativo-dao.sputnikv2.testnet"}' --accountId dokxo.testnet`

### get the contract treasury address
`near view $CONTRACT get_treasury`

### set the contract treasury address
`near call $CONTRACT set_treasury '{"new_account":"joehank.testnet"}' --accountId dokxo.testnet`

### get the fee for buy and sell 
`near view $CONTRACT  get_mint_fee`
### set the fee for buy and sell 
`near call $CONTRACT set_mint_fee '{"mint_fee":0.995}' --accountId dokxo.testnet`

### set a new contract in the whitelist
`near call $CONTRACT add_new_ext_contract '{"address_contract":"minterv2.nativo-minter.testnet","contract_name":"Nativo minter"}' --accountId dokxo.testnet`

## Crear una nueva propuesta de actualizacion desde la DAO(testeado)
`sputnikdao proposal upgrade ./res/Nativo_market_std.wasm $CONTRACT --daoAcc nativo-dao --accountId dokxo.testnet`

## Crear una nueva propuesta para la actualizacion del dueño del market desde la DAO(testeado)
`sputnikdao proposal call  $CONTRACT set_owner_account '{"new_account":"nativo-dao.sputnikv2.testnet"}' --daoAcc nativo-dao --accountId nativo-market.testnet`

## Crear una nueva propuesta para la actualizacion del tesorero del market desde la DAO(testeado)
`sputnikdao proposal call  $CONTRACT set_treasury '{"new_account":"dokxo.testnet"}' --daoAcc nativo-dao --accountId nativo-market.testnet`

### get a specify sale 
`near call $CONTRACT get_sale '{"nft_contract_token":"minterv2.nativo-minter.testnet.7"}' --accountId dokxo.testnet`

### get the sales  by owner 
`near view $CONTRACT get_sales_by_owner_id '{"account_id":"joehank.testnet","from_index":"0","limit":25}'`

### get the storage balance for the account
`near view $CONTRACT storage_balance_of  '{"account_id":"alexiaab.testnet"}'`
### get the total sales for a contract address.
`near view $CONTRACT get_sales_by_nft_contract_id '{"nft_contract_id":"hardtest.nativo-minter.testnet","from_index":"0","limit":25}'`

### update the price for a token_id
`near call $CONTRACT update_price '{"nft_contract_id":"mktstandard.testnet","token_id": "227","price":"10000000000000000000000"}' --account_id joehank.testnet --depositYocto 1`

### make a offer for a token 
`near call $CONTRACT offer '{"account_id":"dokxo.testnet","nft_contract_id":"minterv2.nativo-minter.testnet","token_id":"7"}' --accountId nativo-market.testnet --deposit 1 --gas=300000000000000`

### make a deposit for storage payment 
`near call $CONTRACT storage_deposit '{"account_id":"dokxo.testnet"}' --accountId dokxo.testnet —deposit 0.1`

### remove the token from the market
`near call $CONTRACT remove_sale '{"nft_contract_id":"hardtest.nativo-minter.testnet","token_id": "2085"}' --account_id dokxo.testnet --depositYocto 1`
 





# Offert out of the market


### 1 this offer can be done without need to be listed in sales
`near call $CONTRACT add_offer  '{"nft_contract_id":"minterv2.nativo-minter.testnet","token_id":"10","owner_id":"darkdokxo.testnet"}' --accountId dokxo.testnet  --deposit 0.001 ` 

### 2 this commad returns the offer for a token if exists
`near view $CONTRACT get_offer '{"nft_contract_id":"minterv2.nativo-minter.testnet","token_id":"10"}' --accountId dokxo.testnet`

### 3.1 this commad can be called by the owner or the bidder,it returns the amount payed to the bidder and remove the offer
`near call $CONTRACT delete_offer '{"nft_contract_id":"hardtest.nativo-minter.testnet","token_id":"2085"}' --accountId dokxo.testnet  --depositYocto 1 `

### 3.2 list as sales and accept

`near call hardtest.nativo-minter.testnet nft_approve '{"token_id":"2085","account_id":"dev-1652292755849-85527977565434","msg":"{\"market_type\":\"accept_offer\",\"price\":\"1000000000000000000000\",\"title\":\"a planes img\",\"media\":\"bafybeighiaft7p4kjo34iq3blwv4jpde3jvwu2bmw3dlt7r5cqwdkp37xu\",\"creator_id\":\"dokxo.testnet\"}"}' --accountId dokxo.testnet --deposit 0.1 --gas=300000000000000`

 

 # Sales views 
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


### get a specify Sales

`near view $CONTRACT get_sale '{"nft_contract_token":"minterv2.nativo-minter.testnet.10"}'  --accountId dokxo.testnet `



# Offers views 
### get the total suply of offers
`near view $CONTRACT get_supply_offers  --accountId dokxo.testnet`

### get the total offers's by owner   
`near view $CONTRACT get_supply_offers_by_owner_id '{"account_id":"dokxo.testnet"}' --accountId dokxo.testnet`
### get the  offers by bidder
`near view $CONTRACT get_supply_offers_by_bidder_id '{"account_id":"darkdokxo.testnet"}' --accountId dokxo.testnet`

### get the  offers by owner
`near view $CONTRACT get_offers_by_owner_id '{"account_id":"darkdokxo.testnet","from_index":"0","limit":10 }' --accountId dokxo.testnet`

### get the  offers by bidder
`near view $CONTRACT get_offers_by_bidder_id '{"account_id":"joehank.testnet","from_index":"0","limit":18 }' --accountId dokxo.testnet`

`near view $CONTRACT get_offer_bidder '{"bidder_id":"dokxo.testnet"}' --accountId dokxo.testnet`



### get the total  offers's supply by contract  
`near view $CONTRACT get_supply_offers_by_nft_contract_id '{"nft_contract_id":"minterv2.nativo-minter.testnet"}' --accountId dokxo.testnet`

### get the total  offers's by contract  
`near view $CONTRACT get_offers_by_nft_contract_id '{"nft_contract_id":"minterv2.nativo-minter.testnet","from_index":"0","limit":10 }' --accountId dokxo.testnet`



near call $CONTRACT get_owner '{"nft_contract_id":"minterv2.nativo-minter.testnet","token_id":"2"}' --accountId dokxo.testnet


near view minterv2.nativo-minter.testnet nft_token '{"token_id":"2"}' --accountId dokxo.testnet


near call $CONTRACT update_owner_from_minter '{"nft_contract_id":"minterv2.nativo-minter.testnet","token_id":"9"}' --accountId dokxo.testnet --gas=300000000000000