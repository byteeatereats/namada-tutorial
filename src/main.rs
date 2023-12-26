use async_std::fs;

use std::{
    path::{Path, PathBuf},
    str::FromStr,
    vec,
};

use tendermint_rpc::HttpClient;

use namada_sdk::{
    args::TxBuilder,
    bip39::{Mnemonic, MnemonicType},
    core::types::{address::Address, chain::ChainId, key::SchemeType},
    io::NullIo,
    masp::fs::FsShieldedUtils,
    wallet::{fs::FsWalletUtils, DerivationPath},
    zeroize::Zeroizing,
    Namada, NamadaImpl,
};

#[derive(Clone)]
struct Account {
    pub address: Address,
    pub mnemonic: Mnemonic,
}

// Generate the given number of accounts
async fn gen_accounts(namada: &mut impl Namada, count: usize) -> Vec<Account> {
    let mut accounts: Vec<Account> = vec![];

    // Create the given number of accounts
    for i in 0..count {
        let mnemonic = Mnemonic::new(MnemonicType::Words24, namada_sdk::bip39::Language::English);
        let derivation_path = DerivationPath::from_str("m/44'/877'/0'/0'/0'")
            .expect("unable to parse derivation path");
        let alias = format!("default_{}", i);

        let (_key_alias, sk) = namada
            .wallet_mut()
            .await
            .derive_key_from_mnemonic_code(
                SchemeType::Ed25519,
                Some(alias),
                false,
                derivation_path,
                Some((mnemonic.clone(), Zeroizing::new("".to_owned()))),
                None,
            )
            .expect("unable to derive key from mnemonic code");

        accounts.push(Account {
            address: Address::from(&sk.to_public()),
            mnemonic: mnemonic.clone(),
        });
    }

    // Save the new accounts in the wallet
    namada.wallet().await.save().expect("unable to save wallet");
    // Return the generated accounts
    accounts
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Setup client
    let http_client = HttpClient::new("https://rpc.luminara.icu/").unwrap();
    // Remove wallet file if it exists
    let _ = fs::remove_file("wallet").await;
    // Setup wallet storage
    let wallet = FsWalletUtils::new(PathBuf::from("wallet"));
    // Setup shielded context storage
    let shielded_ctx = FsShieldedUtils::new(Path::new("masp/").to_path_buf());
    // Setup the Namada context
    let mut namada = NamadaImpl::new(http_client, wallet, shielded_ctx, NullIo)
        .await
        .expect("unable to construct Namada object")
        .chain_id(ChainId::from_str("luminara.857cf638d323bbae2ed94").unwrap());
    // Generate accounts
    let accounts = gen_accounts(&mut namada, 2).await;
    for account in &accounts {
        println!(
            "Address: {:?} - Mnemonic: {:?}",
            account.address, account.mnemonic,
        );
    }
    Ok(())
}

