//Code for retrieving information about one or multiple transactions on mempool.space.

use reqwest::header;
use hex;
use tokio::time::interval;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

fn get_client() -> reqwest::Client {
    let headers = header::HeaderMap::new();

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    client
}

struct interestingTx {
    txid: String,
    fees: u64,
}

#[tokio::main]
async fn main() {

    let mut tab_interesting_tx=Vec::new();

    let txids_url = "https://mempool.space/testnet/api/mempool/txids";

    let client = get_client();
    let txids_response = client.get(txids_url).send().await.unwrap();

    if txids_response.status().is_success() {
        let txids = txids_response.json::<Vec<String>>().await.unwrap();

        //while après
        for txid in &txids {
            //let txid = &txids[i];
            let tx_url = format!("https://mempool.space/testnet/api/tx/{}", &txid);
        
            let tx_response = client.get(&tx_url).send().await.unwrap();
            if tx_response.status().is_success() {
                let tx_json = tx_response.json::<serde_json::Value>().await.unwrap();
        
                let vin = tx_json["vin"].as_array().unwrap();
                let fees_origin = tx_json["fee"].as_u64().unwrap();
                for input in vin {
                    let scriptsig = input["scriptsig"].as_str().unwrap_or("");
                    let witness = input["witness"].as_array().map(|arr| arr.to_vec()).unwrap_or_else(Vec::new);
                    
                    let first_witness = &witness.first();
                    let mut decoded_witness = Vec::new();

                    //I print the decoded witness here
                    for witness_value in first_witness {
                        if let Some(witness_str) = witness_value.as_str() {
                            decoded_witness = hex::decode(witness_str).expect("Failed to decode witness");
                            //println!("decoded witness: {:?}", &decoded_witness);
                        }
                    }
                    
                    println!("txid: {}", &txid);

                        if let Some(last_value) = decoded_witness.last() {
                            //if last_value == &130 {
                            if last_value != &1 {
                                println!("La dernière valeur du premier vecteur de witness est différente de 1 : {}", last_value);
                                tab_interesting_tx.push(interestingTx{txid: txid.to_string(), fees: tx_json["fee"].as_u64().unwrap()});
                                
                                println!("fees: {}", fees_origin);
                            }
                            
                        
                        
                            
                            
                             //else {
                                //println!("La dernière valeur du premier vecteur de witness est égale à 1.");
                           // }
                        } //else {
                            //println!("Le premier vecteur de witness est vide.");
                      //  }
                        

                    //I print the scriptsig and witness here
                   // println!("scriptsig: {:?}", scriptsig);
                    //println!("witness: {:?}", witness);
                    
                }
            } else {
                println!("Request failed with status code: {}", tx_response.status());
            }
        }
    } else {
        println!("Request failed with status code: {}", txids_response.status());
    }
}