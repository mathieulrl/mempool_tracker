# Mempool Tracker

**School Project:** Bitcoin Mempool Tracking of Anomalies in Transaction Scripts

## Project Overview 📋

The goal of this project is to:

1. Retrieve information about one or several transactions in a mempool.
   - We chose to use [mempool.space](https://mempool.space) and its APIs (see `main.rs`).

2. Build a transaction from scratch.
   - We chose the Electrum wallet for this purpose (see `transaction2.rs`).

3. Convert the P2WPKH format provided by Electrum (when requesting the private key) into the WPKH format required in the `transaction2.rs` code (functionality not used in the final version).

## Project Execution :chart_with_upwards_trend:

To verify the effectiveness of our code in identifying transactions with anomalies, we will follow these steps:

1. **Create a Transaction with Anomalies:**
   - Introduce anomalies in a transaction, such as an AnyoneCanSpend transaction.

2. **Track the Transaction:**
   - Utilize our tracker to monitor the transaction.

3. **Recover Information about the Transaction:**
   - Retrieve details like fees and UTXOs associated with the transaction.

4. **Recreate the Transaction with Modifications:**
   - Build the same transaction but with higher fees to outpace the original transaction in the mempool.
   - Modify the destination address to recover the funds.

### Why Create a Fake Transaction with Anomalies?

On the Testnet, there is less activity compared to the real Bitcoin network. Therefore, finding interesting transactions can be challenging. By creating a fake transaction with anomalies, we can effectively test and evaluate our code in a controlled environment.

### Difficulties encountered 🚧

   - The literature about our project is limited and hard to find and above all in Rust
   - The issues about descriptors was more complicated as expected. We struggle to convert our private keys into a descriptor for the transaction
---

# Description of the project elements 🔍

## main

This Rust code retrieves information about transactions from the testnet mempool.space API, focusing on transactions with specific criteria.

#### HTTP Client Setup

The `get_client` function initializes a reqwest HTTP client with default headers and a user agent.

#### Data Structure

The `interestingTx` struct defines a structure to store information about transactions deemed interesting, specifically the transaction ID (`txid`) and fees.

#### Main Function

A vector `tab_interesting_tx` is created to store instances of interesting transactions.

#### Transaction Retrieval

- The program makes an HTTP GET request to the mempool.space testnet API to retrieve a list of transaction IDs (`txids`).
- For each transaction ID, it constructs a URL to fetch detailed transaction information.
- It makes another HTTP GET request to retrieve the transaction details.
- For each input and output in the transaction, it extracts the `scriptsig`, `witness` and `scriptPubKey` informations.
- If the last value of the first witness vector is not 1 (we seek a value == 138), it considers the transaction interesting and adds it to `tab_interesting_tx`.
- If any output of the transaction have the beginning of its ScriptPubKey in hexadecimal starting by "51" (=Anyone Can Spend), it also considers the transaction interesting and adds it to `tab_interesting_tx`.
- It prints relevant information, including the transaction ID and fees.

## transaction

This is code, that we first try to just build a "Normal" transaction from scratch.

## transaction2

This Rust code creates a Bitcoin transaction from scratch using the Electrum wallet. The program generates a new wallet, obtains a new address, and then builds a transaction "AnyoneCanSpend" to send funds to a specified address. Additionally, it modifies the script pubkey of one of the transaction outputs for demonstration purposes. The transaction is signed, finalized, and broadcasted to the Bitcoin testnet.

## wif 

This code handles the conversion of a Bitcoin private key to its corresponding Wallet Import Format (WIF). It utilizes the bitcoin crate and extracts the private key from environment variables. The resulting WIF is printed to the console. 
This code is unused in the final version of the project.

## wpkh

This code converts a Pay-to-Witness-Public-Key-Hash (P2WPKH) Bitcoin address to its corresponding Witness Public Key Hash (WPKH) descriptor. The main function initializes an address, calls the conversion function, and prints the resulting WPKH descriptor. 
This code is unused in the final version of the project.

## steal_tx

This code recreates a specific transaction found by the tracker with higher fees. It creates a new Bitcoin transaction on the Testnet using the Electrum wallet, generating a new wallet, obtaining an address, and building a transaction with increased fees compared to a previously tracked transaction. The finalized transaction is then signed and broadcasted to the Bitcoin Testnet. It's important to note that, as of the current implementation, there are challenges in successfully completing the code, and this process hasn't been automated : you have to manually enter the id of and interesting transaction tracked previously in order to steal the UTXO signed in "Anyone Can Spend".


# Getting Started 🚀

## Command lines:   
   - main.rs : cargo run --bin main
   - transaction.rs : cargo run --bin transaction
   - transaction2.rs : cargo run --bin transaction2
   - steal_tx.rs : cargo run --bin steal
   - wpkh.rs : cargo run --bin wpkh
   - wif.rs : cargo run --bin wif


# Example of an UTXO successfully "stolen" :

## Transaction "Anyone Can Spend" created with transaction2.rs :
![image](https://github.com/mathieulrl/mempool_tracker/assets/95310781/901e1ee1-3cab-43da-95df-ce9db70f81d7)
![image](https://github.com/mathieulrl/mempool_tracker/assets/95310781/280294c1-6d5e-407f-b449-b9c8d04b053d)

## This transaction is successfully retrieved in the Interesting Transactions table :
![image](https://github.com/mathieulrl/mempool_tracker/assets/95310781/19ddce54-9e4a-49da-873a-69159ee72b8d)

## Then, we manage to steal the Anyone Can Spend output of the transaction by creating a new transaction with higher fees :
![image](https://github.com/mathieulrl/mempool_tracker/assets/95310781/d7553a8d-fc82-49af-9c2d-294c34706d93)
![image](https://github.com/mathieulrl/mempool_tracker/assets/95310781/c38bdfda-f900-4d1f-bae6-fc2901070fa5)


