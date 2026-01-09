use ethers::prelude::*;
use std::convert::TryFrom;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";

    println!("正在连接到 Arbitrum Sepolia 测试网...");
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let block_num = provider.get_block_number().await?;
    println!("Hello Web3! 成功连接到 Arbitrum 链上。");
    println!("当前区块高度 (Block Number): {}", block_num);

    Ok(())
}
