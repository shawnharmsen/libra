// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    dev_api_client::DevApiClient,
    shared::{self, build_move_package, NetworkHome, LATEST_USERNAME},
};
use anyhow::{anyhow, Result};
use diem_crypto::PrivateKey;
use diem_sdk::{
    transaction_builder::TransactionFactory,
    types::{
        transaction::{ModuleBundle, TransactionPayload},
        LocalAccount,
    },
};
use diem_types::{chain_id::{ChainId, NamedChain}, transaction::authenticator::AuthenticationKey};
use generate_key::load_key;
use std::path::Path;
use url::Url;

/// Deploys shuffle's main Move Package to the sender's address.
pub async fn handle(network_home: &NetworkHome, project_path: &Path, url: Url, chain_name: NamedChain, use_mnem: bool) -> Result<()> {
    if !network_home.key_path_for(LATEST_USERNAME).exists() {
        return Err(anyhow!(
            "An account hasn't been created yet! Run shuffle account first."
        ));
    }
    
    ///////// 0L ////////
    let account_key = if use_mnem {
      let (_, b, c) = ol_keys::wallet::get_account_from_prompt();
      c.get_private_key(&b)?
    } else {
      load_key(network_home.key_path_for(LATEST_USERNAME))

    };
    ///////// end 0L ////////

    println!("Using Public Key {}", &account_key.public_key());
    let address = AuthenticationKey::ed25519(&account_key.public_key()).derived_address();
    println!("Sending txn from address {}", address.to_hex_literal());

    let client = DevApiClient::new(reqwest::Client::new(), url)?;
    let seq_number = client.get_account_sequence_number(address).await?;
    let mut account = LocalAccount::new(address, account_key, seq_number);

    deploy(&client, &mut account, project_path, chain_name).await
}

pub async fn deploy(
    client: &DevApiClient,
    account: &mut LocalAccount,
    project_path: &Path,
    chain_name: NamedChain,
) -> Result<()> {
    let compiled_package = build_move_package(
        project_path.join(shared::MAIN_PKG_PATH).as_ref(),
        &account.address(),
    )?;
    for module in compiled_package
        .transitive_compiled_modules()
        .compute_dependency_graph()
        .compute_topological_order()?
    {
        let module_id = module.self_id();
        if module_id.address() != &account.address() {
            println!("Skipping Module: {}", module_id);
            continue;
        }
        println!("Deploying Module: {}", module_id);
        let mut binary = vec![];
        module.serialize(&mut binary)?;

        let hash = send_module_transaction(client, account, binary, chain_name).await?;
        client.check_txn_executed_from_hash(hash.as_str()).await?;
    }

    Ok(())
}

async fn send_module_transaction(
    client: &DevApiClient,
    account: &mut LocalAccount,
    module_binary: Vec<u8>,
    chain_name: NamedChain,
) -> Result<String> {
    let factory = TransactionFactory::new(ChainId::new(chain_name.id()));
    let publish_txn = account.sign_with_transaction_builder(factory.payload(
        TransactionPayload::ModuleBundle(ModuleBundle::singleton(module_binary)),
    ));
    let bytes = bcs::to_bytes(&publish_txn)?;
    let json = client.post_transactions(bytes).await?;
    let hash = DevApiClient::get_hash_from_post_txn(json)?;
    Ok(hash)
}
