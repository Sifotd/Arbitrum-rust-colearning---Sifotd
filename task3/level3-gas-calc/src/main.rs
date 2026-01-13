use ethers::prelude::*;
use ethers::utils::format_units;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    let provider = Provider::<Http>::try_from(rpc_url)?;

    println!("正在连接 Arbitrum 测试网并获取实时 Gas 信息...");

  
    let gas_price = provider.get_gas_price().await?;

    
    let gas_limit = 21000; 


    let estimated_fee = gas_price * gas_limit;
   
    let gas_price_gwei = format_units(gas_price, "gwei")?;
    
    let estimated_fee_eth = format_units(estimated_fee, "ether")?;

    println!("------------------------------------------------");
    println!("当前网络 Gas Price: {} Gwei", gas_price_gwei);
    println!("基础转账 Gas Limit: {}", gas_limit);
    println!("------------------------------------------------");
    println!(">>>> 预估转账手续费: {} ETH", estimated_fee_eth);
    println!("------------------------------------------------");

    Ok(())
}