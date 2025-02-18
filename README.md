# What's this?

This repository is example for showing how to mint NFT in solana and manage it.

This contains:
- Mint new NFT
- Transfer NFT
- Update NFT metadata

# Platform Tools Installed and Tested
- Anchor: `anchor-cli 0.30.1`
- Solana: `solana-cli 2.0.16 (src:f5a88ce9; feat:607245837, client:Agave)`
- Rust: `rustc 1.80.1 (3f5fd8dd4 2024-08-06)`

# Preparation
- Prepare metadata json file

    Sample: https://moccasin-urgent-shrew-838.mypinata.cloud/ipfs/bafkreiceu3llxj3px4mtylh245xpskecz7qxfhp744et7w4wt6znqcvipm

- Install platform tools

# How to run?
- Clone this repo
    ```
    git clone https://github.com/toygr/solana-nft-example.git
    cd solana-nft-example
    ```
- Sync keys
    ```
    anchor keys list
    anchor keys sync
    ```
- Build
    ```
    anchor build
    ```
- Test
    ```
    anchor test
    ```

    (Optionally) You can test without deploying

    ```
    anchor test --skip-deploy
    ```