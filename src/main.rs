use std::str::FromStr;
use std::{thread, time::Duration};
use web3::helpers as w3h;
use web3::types::{BlockId, BlockNumber, TransactionId, H160};
use indicatif::ProgressBar;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct TX {
    from: String,
    to: String,
    value: String
}

#[tokio::main]
async fn main() -> web3::Result<()> {

    // wss://rpc.mainnet.pulsechain.com/ws/v1/

    let web3httpclient = web3::transports::Http::new("https://rpc.v2b.testnet.pulsechain.com").unwrap();
    let web3s = web3::Web3::new(web3httpclient);
    let block_counter: u64 = 14442369;
    let block_counter_end: u64 = 15793697;
    let block_to_check = block_counter_end - block_counter;

    let contract_address = "0x07895912f3ab0e33ab3a4cefbdf7a3e121eb9942";
    let pb = ProgressBar::new(block_to_check);

    let mut txs: Vec<TX> = vec![];

    for current_block_index in block_counter..block_counter_end {
        //println!("current_block_index {}", current_block_index);

        let latest_block = web3s
        .eth()
        .block(BlockId::Number(BlockNumber::Number(web3::types::U64::from(current_block_index))))
        .await
        .unwrap()
        .unwrap();

        for transaction_hash in latest_block.transactions {
            let tx = match web3s
                .eth()
                .transaction(TransactionId::Hash(transaction_hash))
                .await
            {
                Ok(Some(tx)) => tx,
                _ => {
                    println!("An error occurred");
                    continue;
                }
            };
            let from_addr = tx.from.unwrap_or(H160::zero());
            let to_addr = tx.to.unwrap_or(H160::zero());
            let wallet2: H160 = H160::from_str(contract_address).unwrap();

            if to_addr == wallet2 {

                let ctx = TX {
                    from:  w3h::to_string(&from_addr),
                    to:  w3h::to_string(&to_addr),
                    value: tx.value.to_string(),
                };

                //let serialized_ctx = serde_json::to_string(&ctx).unwrap();
                //println!("{}", serialized_ctx);
                txs.push(ctx);
            }
        }
        pb.inc(1);
    thread::sleep(Duration::from_millis(5));
    }
    pb.finish_with_message("done");
        // Save the JSON structure into the other file.
        std::fs::write(
            "result.txt",
            serde_json::to_string_pretty(&txs).unwrap(),
        )
        .unwrap();
    Ok(())
}