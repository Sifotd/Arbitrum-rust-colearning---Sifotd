use ethers::prelude::*;
use std::convert::TryFrom;
use std::sync::Arc;


abigen!(
    ERC20Contract,
    r#"[
        function name() external view returns (string)
        function symbol() external view returns (string)
        function decimals() external view returns (uint8)
        function totalSupply() external view returns (uint256)
    ]"#
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    let provider = Provider::<Http>::try_from(rpc_url)?;
 
    let client = Arc::new(provider);

    println!("正在连接 Arbitrum Sepolia 节点...");
 
    let contract_address: Address = "0x980B62Da83eFf3D4576C647993b0c1D7faf17c73".parse()?;


    let contract = ERC20Contract::new(contract_address, client.clone());

    println!("正在读取合约信息，地址: {:?}", contract_address);
    println!("-------------------------------------------");

  
    let name = contract.name().call().await?;
    println!("合约名称 (Name): {}", name);

    let symbol = contract.symbol().call().await?;
    println!("代币符号 (Symbol): {}", symbol);

    let decimals = contract.decimals().call().await?;
    println!("代币精度 (Decimals): {}", decimals);


    let total_supply = contract.total_supply().call().await?;
    println!("总供应量 (Total Supply): {}", total_supply);
    
    println!("-------------------------------------------");
    println!("🎉 合约交互成功！");

    Ok(())
}