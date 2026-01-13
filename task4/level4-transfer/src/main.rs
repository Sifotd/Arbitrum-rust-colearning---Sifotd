use ethers::prelude::*;
use std::convert::TryFrom;
use std::str::FromStr;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();


    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    let provider = Provider::<Http>::try_from(rpc_url)?;


    let private_key = env::var("PRIVATE_KEY").expect("请在 .env 文件中配置 PRIVATE_KEY");
    let wallet: LocalWallet = private_key.parse()?;

    let wallet = wallet.with_chain_id(421614u64);

    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    println!("当前账户: {:?}", wallet.address());

  
  let to_address = Address::from_str("0xc394175f173950598ddcc608b1bf09a92d7a02a0")?;
   

    let transfer_amount = U256::from_dec_str("100000000000000")?;

    println!("正在获取网络 Gas 价格...");

   
    let current_gas_price = provider.get_gas_price().await?;
 
    let adjusted_gas_price = current_gas_price * 120 / 100;

    println!("网络原始 Gas Price: {:?}", current_gas_price);
    println!("调整后 Gas Price (+20%): {:?}", adjusted_gas_price);


    let tx = TransactionRequest::new()
        .to(to_address)
        .value(transfer_amount)
        .gas_price(adjusted_gas_price); 

    println!("开始发送交易...");


    let pending_tx = client.send_transaction(tx, None).await?;
    
    println!("交易已广播! Hash: {:?}", pending_tx.tx_hash());
    println!("正在等待链上确认 (这可能需要几秒钟)...");


    let receipt = pending_tx.await?;

   
    match receipt {
        Some(receipt) => {
            println!("-----------------------------------------");
            println!("✅ 交易成功上链！");
            println!("交易哈希: {:?}", receipt.transaction_hash);
            println!("区块高度: {:?}", receipt.block_number);
            println!("浏览器查询: https://sepolia.arbiscan.io/tx/{:?}", receipt.transaction_hash);
            println!("-----------------------------------------");
        },
        None => {
            println!("❌ 交易未被确认，可能是网络延迟，请稍后在浏览器查询哈希。");
        }
    }

    Ok(())
}