use clap::Parser;
use core::fmt;
use serde::Deserialize;
use std::{error::Error, fmt::Formatter};

#[derive(Debug, Deserialize)]
pub struct HeliusLogsSubscribeResponse {
    pub params: Option<HeliusParams>,
}

#[derive(Debug, Deserialize)]
pub struct HeliusParams {
    pub result: HeliusParamsResult,
}

#[derive(Debug, Deserialize)]
pub struct HeliusParamsResult {
    pub value: HeliusParamsResultValue,
}

#[derive(Debug, Deserialize)]
pub struct HeliusParamsResultValue {
    pub signature: String,
    pub logs: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct HeliusGetTransactionResponse {
    pub result: Option<HeliusResult>,
}

#[derive(Debug, Deserialize)]
pub struct HeliusResult {
    pub transaction: HeliusTransaction,
    pub meta: HeliusMeta,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusMeta {
    pub post_token_balances: Vec<HeliusPostTokenBalances>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusPostTokenBalances {
    account_index: u32,
    ui_token_amount: HeliusUiTokenAmount,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusUiTokenAmount {
    pub ui_amount: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct HeliusTransaction {
    pub message: HeliusTransactionMessage,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusTransactionMessage {
    pub account_keys: Vec<String>,
    pub instructions: Vec<HeliusInstruction>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusInstruction {
    pub accounts: Vec<u32>,
    pub program_id_index: u32,
}

#[derive(Debug, Clone)]
pub struct JupiterDcaData {
    pub wallet: String,
    pub input_mint: String,
    pub input_amount: f64,
    pub output_mint: String,
}

#[derive(Debug)]
pub struct DcaResult {
    pub signature: String,
    pub dca_data: JupiterDcaData,
    pub input_ticker: String,
    pub output_ticker: String,
}

#[derive(Debug, Deserialize)]
pub struct HeliusGetAssetResponse {
    pub result: Option<HeliusDasResult>,
}

#[derive(Debug, Deserialize)]
pub struct HeliusDasResult {
    pub content: HeliusDasContent,
}

#[derive(Debug, Deserialize)]
pub struct HeliusDasContent {
    pub metadata: HeliusDasMetadata,
}

#[derive(Debug, Deserialize)]
pub struct HeliusDasMetadata {
    pub symbol: String,
}

impl HeliusGetTransactionResponse {
    pub fn process_data(&self) -> Result<JupiterDcaData, Box<dyn Error>> {
        let result = self.result.as_ref().ok_or("Missing result field.")?;

        let accounts = &result.transaction.message.account_keys;

        let instructions = &result.transaction.message.instructions;

        const DCA_ADDRESS: &str = "DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M";
        let account_indices = &instructions
            .iter()
            .find(|ix| {
                let idx = ix.program_id_index as usize;
                accounts[idx] == DCA_ADDRESS
            })
            .map(|ix| &ix.accounts)
            .ok_or("Couldn't find account indices.")?;

        let wallet_idx = account_indices[1] as usize;
        let input_mint_idx = account_indices[3] as usize;
        let output_mint_idx = account_indices[4] as usize;
        let in_ata_idx = account_indices[6] as usize;

        let wallet = accounts[wallet_idx].clone();
        let input_mint = accounts[input_mint_idx].clone();
        let output_mint = accounts[output_mint_idx].clone();

        let input_amount = result
            .meta
            .post_token_balances
            .iter()
            .find(|bal| bal.account_index as usize == in_ata_idx)
            .and_then(|bal| bal.ui_token_amount.ui_amount)
            .ok_or("Couldn't find input token amount")?;

        Ok(JupiterDcaData {
            wallet,
            input_mint,
            input_amount,
            output_mint,
        })
    }
}

impl fmt::Display for DcaResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n{} just opened a Jupiter DCA\n{} {} -> {}\nTx: {}",
            self.dca_data.wallet,
            self.dca_data.input_amount,
            self.input_ticker,
            self.output_ticker,
            self.signature
        )
    }
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, default_value = "5000.0")]
    pub usdc: f64,
    #[arg(long, default_value = "50.0")]
    pub sol: f64,
    #[arg(long, default_value = "false")]
    pub no_filter: bool,
}
