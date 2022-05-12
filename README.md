
### create a subaccount
`near create-account v2.nativo-market.testnet --masterAccount nativo-market.testnet`
### delete a subaccount
`near delete v1.nativo-market.testnet nativo-market.testnet`

### Compile,build and deploy the market contract 
`./build.sh`
### Set the market contract global
`export CONTRACT="v2.nativo-market.testnet" `
export CONTRACT="dev-1652292755849-85527977565434" 
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
`near call $CONTRACT add_offer  '{"nft_contract_id":"minterv2.nativo-minter.testnet","token_id":"7","owner_id":"dokxo.testnet"}' --accountId darkdokxo.testnet  --deposit 0.001 ` 

### 2 this commad returns the offer for a token if exists
`near view $CONTRACT get_offer '{"nft_contract_id":"minterv2.nativo-minter.testnet","token_id":"7"}' --accountId dokxo.testnet`

### 3.1 this commad can be called by the owner or the bidder,it returns the amount payed to the bidder and remove the offer
`near call $CONTRACT delete_offer '{"nft_contract_id":"hardtest.nativo-minter.testnet","token_id":"2085"}' --accountId dokxo.testnet  --depositYocto 1 `

### 3.2 list as sales and accept

`near call hardtest.nativo-minter.testnet nft_approve '{"token_id":"2085","account_id":"dev-1652292755849-85527977565434","msg":"{\"market_type\":\"accept_offer\",\"price\":\"1000000000000000000000\",\"title\":\"a planes img\",\"media\":\"bafybeighiaft7p4kjo34iq3blwv4jpde3jvwu2bmw3dlt7r5cqwdkp37xu\",\"creator_id\":\"dokxo.testnet\"}"}' --accountId dokxo.testnet --deposit 0.1 --gas=300000000000000`

 

#  5 this command only can be called by the owner to accept the offer,paying the royalites,the nfv and resolving the sale
### it only can be called after the token is listed to the market(call this function after  nft_on_approve )
`near call $CONTRACT accept_offer '{"nft_contract_id":"hardtest.nativo-minter.testnet","token_id":"2085"}' --accountId dokxo.testnet  --depositYocto 1  --gas=300000000000000`

# Auctions
### list as sales

`near call minterv2.nativo-minter.testnet nft_approve '{"token_id":"7","account_id":"dev-1652292755849-85527977565434","msg":"{\"market_type\":\"on_sale\",\"price\":\"1000000000000000000000000\",\"title\":\"Elmo te esta vigilando\",\"media\":\"bafybeigp4fyo3umq3teaxy7yx5cfv2uj7fsou2ffxkjocgksaheymvmxja\",\"creator_id\":\"alexiaab.testnet\"}"}' --accountId darkdokxo.testnet --deposit 0.1`

### list as auction
`near call hardtest.nativo-minter.testnet nft_approve '{"token_id":"22908","account_id":"dev-1650923930420-44090012366220","msg":"{\"market_type\":\"on_auction\",\"price\":\"1000000000000000000000000\",\"title\":\"Dark JoeHank\",\"media\":\"bafybeifocdpvwqqlgnq3nx56ran6tynrlyb4pbbzwsrgglulf2gweqmt5m\",\"creator_id\":\"dokxo.testnet\"}"}' --accountId dokxo.testnet --deposit 0.1`








### add a bid
`near call $CONTRACT add_bid '{"nft_contract_id":"minterv2.nativo-minter.testnet","token_id":"7"}' --accountId dokxo.testnet    --deposit 100.1 `
### procees auction
#### the owner can accept a offer it must to add gass by the XCC to the minter
`near call $CONTRACT process_bid '{"nft_contract_id":"hardtest.nativo-minter.testnet","token_id":"2085","response":true}' --accountId dokxo.testnet  --depositYocto 1 --gas=300000000000000`
#### the owner can decline a offer  
`near call $CONTRACT process_bid '{"nft_contract_id":"hardtest.nativo-minter.testnet","token_id":"2085","response":false}' --accountId dokxo.testnet  --depositYocto 1  `
#### the bidder can decline a offer 
#### no one else can accept or decline a offer 







export CONTRACT="dev-1649971578313-51969273503714" 
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/Nativo_market_std.wasm

`near call $CONTRACT new '{"owner_id":"dokxo.testnet"}'  --accountId dokxo.testnet`










near call hardtest.nativo-minter.testnet nft_approve '{"token_id":"9","account_id":"dev-1649875934003-18318891633598","msg":"{\"sale_conditions\":\"10000000000000000000000000\"}"}' --accountId dokxo.testnet --deposit 1



near view $CONTRACT get_sales_by_nft_contract_id '{"nft_contract_id":"hardtest.nativo-minter.testnet"}'

near view $CONTRACT get_sales_by_owner_id '{"account_id":"nativo-market.testnet","from_index":"0","limit":25}'


near call $CONTRACT offer '{"account_id":"nativo-market.testnet","nft_contract_id":"hardtest.nativo-minter.testnet","token_id":"10"}' --accountId dokxo.testnet --deposit 1 --gas=300000000000000

near view $CONTRACT storage_balance_of  '{"account_id":"customnativo.testnet"}'


repeat 10000 {
    near call hardtest.nativo-minter.testnet nft_mint '{"metadata": {"title": "Token de dokxo", "description": "Es una nube jejeje x2", "media": "bafybeiespmva6en5xy3giajcewap3avkypou4ylaqguymrrol2ccumz7le"}, "receiver_id": "dokxo.testnet", "perpetual_royalties": {"nativo-mkt.testnet": 2000}}' --accountId dokxo.testnet --amount 0.1
}  ;

near call hardtest.nativo-minter.testnet nft_approve '{"token_id":"10","account_id":"v1.nativo-market.testnet","msg":"{\"sale_conditions\":\"1000000000000000000000000\"}"}' --accountId dokxo.testnet --deposit 0.1


near view hardtest.nativo-minter.testnet nft_tokens '{"account_id":"customnativo.testnet","from_index":"1419","limit":2}' --accountId dokxo.testnet --gas=300000000000000

near call $CONTRACT pay_the_market_fee '{"price":1000}' --accountId dokxo.testnet

near view $CONTRACT get_supply_sales

repeat 10000 {near call hardtest.nativo-minter.testnet nft_mint '{"metadata": {"title": "Tokens en Masa de dokxo  ", "description": "Es una nube jejeje x2", "media": "bafybeiespmva6en5xy3giajcewap3avkypou4ylaqguymrrol2ccumz7le"}, "receiver_id": "dokxo.testnet", "perpetual_royalties": {"dokxo.testnet": 2000}}' --accountId dokxo.testnet --amount 0.1};

near call $CONTRACT add_bid '{"nft_contract_id":"hardtest.nativo-minter.testnet","token_id":"2085"}' --accountId dokxo.testnet    



near call hardtest.nativo-minter.testnet nft_approve '{"token_id":"22919","account_id":"dev-1651700996675-98618622798947","msg":"{\"market_type\":\"on_sale\",\"price\":\"10000000000000000000000000\",\"title\":\"Dark JoeHank\",\"media\":\"bafybeifocdpvwqqlgnq3nx56ran6tynrlyb4pbbzwsrgglulf2gweqmt5m\",\"creator_id\":\"dokxo.testnet\"}"}' --accountId darkdokxo.testnet --deposit 0.1



near call $CONTRACT halo2 --accountId dokxo.testnet    
