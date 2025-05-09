# USAGE
1. Clone the repo to use it locally with `git clone https://github.com/pbht/solana-dca-websocket-listener.git`
2. cd into the repo with `cd solana-dca-websocket-listener`
3. Create a .env file with environment variable `HELIUS_API_KEY=<YOUR-KEY>` and store in the project root
4. Build with `cargo build --release`
5. Run with `./target/release/solana-dca-websocket-listener` to run the script fully optimized

# INTERESTING OBSERVATIONS
![$1M Fartcoin DCA](assets/fartcoin-1m-dca.png)
![$750K Fartcoin DCA](assets/fartcoin-750k-dca.png)

This wallet opened ~$1.75M worth of DCAs to buy FARTCOIN

# TODO
- [x] Store the signature 
- [x] Store the wallet address that is executing the DCA
- [x] Remove any hardcoded list indices (figure out how the lists are indexed)
- [x] Query tickers from Helius DAS
- [ ] Query $ value of input token (useful for non USDC / SOL inputs)
- [ ] Filter orders above a $ value specified by CLI argument
- [ ] Flag if a DCA was closed