import sys
import os
import solcx # Need solc in PATH
import json

from web3 import Web3

if len(sys.argv) != 2:
    print ("Usage: ", sys.argv[0], " <hex-ink-contract-id-address>")
    sys.exit(1)

ink_ext_address = int(sys.argv[1],16)

with open(os.path.join(os.path.dirname(__file__),"contract.sol"), "r") as file:
    contract_file = file.read()
    
    
solcx.install_solc('0.8.18')
    
compiled_sol = solcx.compile_standard(
    {
        "language": "Solidity",
        "sources": {"contract.sol": {"content": contract_file}},
        "settings": {
            "outputSelection": {
                "*": {
                    "*": ["abi", "metadata", "evm.bytecode", "evm.bytecode.sourceMap"] # output needed to interact with and deploy contract 
                }
            }
        },
    },
    solc_version="0.8.18",
)

#print(compiled_sol)

# get bytecode
bytecode = compiled_sol["contracts"]["contract.sol"]["flipper"]["evm"]["bytecode"]["object"]
# get abi
abi = json.loads(compiled_sol["contracts"]["contract.sol"]["flipper"]["metadata"])["output"]["abi"]


w3 = Web3(Web3.HTTPProvider("http://127.0.0.1:9933"))

print("Is connected:", w3.isConnected())

chain_id = 4369

address = "0xaaafB3972B05630fCceE866eC69CdADd9baC2771"
private_key = "0x01ab6e801c06e59ca97a14fc0a1978b27fa366fc87450e0b65459dd3515b7391" 

Flipper =  w3.eth.contract(abi=abi, bytecode=bytecode)
nonce = w3.eth.getTransactionCount(address)

# build transaction
transaction = Flipper.constructor().buildTransaction(
    {
        "chainId": chain_id,
        "gasPrice": w3.eth.gas_price,
        "from": address,
        "nonce": nonce,
    }
)
# Sign the transaction
sign_transaction = w3.eth.account.sign_transaction(transaction, private_key=private_key)
print("Deploying Contract!")
# Send the transaction
transaction_hash = w3.eth.send_raw_transaction(sign_transaction.rawTransaction)
# Wait for the transaction to be mined, and get the transaction receipt
print("Waiting for transaction to finish...")
transaction_receipt = w3.eth.wait_for_transaction_receipt(transaction_hash)
print(f"Done! Contract deployed to {transaction_receipt.contractAddress}")

deployed_contract = w3.eth.contract(address=transaction_receipt.contractAddress, abi=abi)

print("Calling flip function...")
transaction = deployed_contract.functions.flip(ink_ext_address.to_bytes(33)).buildTransaction(
        {
            "chainId": chain_id, 
            "from": address, 
            "gasPrice": w3.eth.gas_price, 
            "nonce": nonce + 1});
sign_transaction = w3.eth.account.sign_transaction(transaction, private_key=private_key)
transaction_hash = w3.eth.send_raw_transaction(sign_transaction.rawTransaction)
print("Waiting for transaction to finish...")
transaction_receipt = w3.eth.wait_for_transaction_receipt(transaction_hash)




