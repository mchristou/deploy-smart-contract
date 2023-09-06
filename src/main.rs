use anyhow::Result;
use ethers::{
    prelude::*,
    providers::{Http, Provider},
    signers::LocalWallet,
};
use std::{fs::File, io::Read, sync::Arc, time::Duration};

abigen!(HelloWorld, "out/HelloWorld.abi");

fn bytecode() -> Vec<u8> {
    let mut hex_str = Vec::new();
    let mut file = File::open("out/HelloWorld.bin").expect("Failed to open bytecode file");
    file.read_to_end(&mut hex_str)
        .expect("Failed to read bytecode file");

    hex::decode(hex_str).expect("Invalid bytecode")
}

fn wallet() -> LocalWallet {
    std::env::var("PRIVATE_KEY")
        .expect("PRIVATE_KEY is missng from env variables")
        .trim_start_matches("0x")
        .parse::<LocalWallet>()
        .expect("Failed to parse private key to wallet")
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let wallet = wallet().with_chain_id(1337u64);

    let provider =
        Provider::<Http>::try_from("http://localhost:8545")?.interval(Duration::from_millis(10));

    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet));

    let factory = ContractFactory::new(HELLOWORLD_ABI.clone(), bytecode().into(), client.clone());

    let contract = factory
        .deploy("Hello World".to_string())?
        .legacy()
        .send()
        .await?;

    let hello_world = HelloWorld::new(contract.address(), client);

    let msg = hello_world.get_message().legacy().call().await?;
    println!("Init msg: {msg}");

    hello_world
        .update_message("Updated hello world!".to_string())
        .legacy()
        .send()
        .await?;

    let msg = hello_world.get_message().legacy().call().await?;
    println!("Updated msg: {msg}");

    Ok(())
}
