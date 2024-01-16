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