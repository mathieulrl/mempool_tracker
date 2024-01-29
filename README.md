# Mempool Tracker

**School Project:** Bitcoin Mempool Tracking of Anomalies in Transaction Scripts

## Project Overview 📋

The goal of this project is to:

1. Retrieve information about one or several transactions in a mempool.
   - We chose to use [mempool.space](https://mempool.space) and its APIs (see `main.rs`).

2. Build a transaction from scratch.
   - We chose the Electrum wallet for this purpose (see `transaction2.rs`).

3. Convert the P2WPKH format provided by Electrum (when requesting the private key) into the WPKH format required in the `transaction2.rs` code.

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
   - The issues about descriptors was more complicated as expected. We struggle to convert our private/puclic keys into a descriptor for the transaction
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
- For each input in the transaction, it extracts the `scriptsig` and `witness` information.
- If the last value of the first witness vector is not 1, it considers the transaction interesting and adds it to `tab_interesting_tx`.
- It prints relevant information, including the transaction ID and fees.

## transaction2

This Rust code creates a Bitcoin transaction from scratch using the Electrum wallet. The program generates a new wallet, obtains a new address, and then builds a transaction to send funds to a specified address. Additionally, it modifies the script pubkey of one of the transaction outputs for demonstration purposes. The transaction is signed, finalized, and broadcasted to the Bitcoin testnet.

## wif

This code handles the conversion of a Bitcoin private key to its corresponding Wallet Import Format (WIF). It utilizes the bitcoin crate and extracts the private key from environment variables. The resulting WIF is printed to the console.

## wpkh

This code converts a Pay-to-Witness-Public-Key-Hash (P2WPKH) Bitcoin address to its corresponding Witness Public Key Hash (WPKH) descriptor. The main function initializes an address, calls the conversion function, and prints the resulting WPKH descriptor. 