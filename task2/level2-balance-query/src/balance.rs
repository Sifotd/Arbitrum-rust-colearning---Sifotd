use ethers::prelude::*;
use ethers::utils::format_units; // 引入单位转换工具
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    let provider = Provider::<Http>::try_from(rpc_url)?;

    let wallet_address = "0xef7ec9c985acfa77839d927182aa5b10714f6132".parse::<Address>()?; 

    println!("正在查询地址: {:?}", wallet_address);  
    let balance_wei = provider.get_balance(wallet_address, None).await?;
    let balance_eth = format_units(balance_wei, "ether")?;
  
    println!("--------------------------------");
    println!("账户余额 (Wei): {}", balance_wei);
    println!("账户余额 (ETH): {} ETH", balance_eth);
    println!("--------------------------------");

    Ok(())
}