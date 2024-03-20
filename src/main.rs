use std::fs;
use std::io::Write;
use dotenv::dotenv;
use bip39::{Mnemonic, MnemonicType, Language};
use ethers::{
    signers::{coins_bip39::English, MnemonicBuilder, Signer},
    prelude::{Provider, Middleware, Http},
};

async fn search(prov: &Provider<Http>) {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    let wallet = MnemonicBuilder::<English>::default().phrase(mnemonic.phrase()).build().unwrap();

    let balance = prov.get_balance(wallet.address(), None).await.unwrap();
    
    if !balance.is_zero() {
        let _ = fs::write(format!("./wallets/{}.txt", wallet.address()), mnemonic.phrase());
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();

    let rpc_url: String = std::env::var("RPC_URL").unwrap();  
    let provider = Provider::<Http>::try_from(&rpc_url).unwrap();

    println!("rover on.");

    let mut i = 0;
    loop { 
        search(&provider).await;
        print!("\rchecked: {}", { i += 1; i }); 
        let _ = std::io::stdout().flush();
    };
}