use crate::types::{HeliusGetAssetResponse, HeliusGetTransactionResponse};
use reqwest::{self, Client};
use serde_json::{self, json};
use std::error::Error;

pub async fn get_transaction(
    client: &Client,
    url: &String,
    sx: &String,
) -> Result<HeliusGetTransactionResponse, Box<dyn Error>> {
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTransaction",
        "params": [
            sx,
            "json"
        ]
    });

    let response = client.post(url).json(&body).send().await?.text().await?;

    let parsed_response: HeliusGetTransactionResponse = serde_json::from_str(response.as_str())?;

    Ok(parsed_response)
}

pub async fn get_ticker(
    client: &Client,
    url: &String,
    mint: &String,
) -> Result<String, Box<dyn Error>> {
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getAsset",
        "params": {
            "id": mint
        }
    });

    let response = client.post(url).json(&body).send().await?.text().await?;

    let parsed_response: HeliusGetAssetResponse = serde_json::from_str(response.as_str())?;
    let symbol = parsed_response
        .result
        .ok_or("Error parsing DAS result.")?
        .content
        .metadata
        .symbol;

    Ok(symbol)
}
