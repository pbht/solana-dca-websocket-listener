use std::{env, error::Error};
use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use reqwest::Client;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use serde_json::json; 

mod types;
mod utils;
use types::{DcaResult, HeliusLogsSubscribeResponse};
use utils::{get_ticker, get_transaction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let helius_api_key = env::var("HELIUS_API_KEY")?;
    let ws_url = format!("wss://mainnet.helius-rpc.com/?api-key={}", &helius_api_key);
    let http_url = format!("https://mainnet.helius-rpc.com?api-key={}", &helius_api_key);

    let (mut ws_stream, _) = connect_async(ws_url.as_str()).await?;
    let client = Client::new();

    println!("Connected to Helius WebSocket.");

    let subscribe_payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "logsSubscribe",
        "params": [
            {
                "mentions": ["DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M"]
            },
            {
                "commitment": "finalized"
            }
        ]
    });

    let payload_str = subscribe_payload.to_string();
    ws_stream.send(Message::Text(payload_str)).await?;

    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let parsed_text: HeliusLogsSubscribeResponse = serde_json::from_str(&text)?;
                if let Some(params) = parsed_text.params {
                    let tx_contains_open_dca_v2 = params.result.value.logs.iter().any(|log| log.contains("OpenDcaV2"));
                    if !tx_contains_open_dca_v2 { continue; }

                    let sx = params.result.value.signature;
                    let dca_result= get_transaction(&client, &http_url, &sx).await?.process_data();
                    
                    match dca_result {
                        Ok(dca_data) => {
                            let input_ticker = get_ticker(&client, &http_url, &dca_data.input_mint).await?;
                            let output_ticker = get_ticker(&client, &http_url, &dca_data.output_mint).await?;

                            // Filter USDC trades >= 10,000
                            if let "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v" = dca_data.input_mint.as_str() {
                                if dca_data.input_amount >= 9999.99 {
                                    let dca = DcaResult {
                                        signature: sx,
                                        dca_data,
                                        input_ticker,
                                        output_ticker
                                    };
                                    println!("{}", dca);
                                }
                            }

                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }

                }
            }
            
            Ok(Message::Close(_)) => {
                println!("Server closed connection.");
                break;
            }

            Ok(_) => {}
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }

    println!("Disconnected.");
    Ok(())
}