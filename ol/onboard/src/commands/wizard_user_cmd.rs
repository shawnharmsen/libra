//! `version` subcommand

#![allow(clippy::never_loop)]

use ol_keys::wallet;
use ol_types::block::Block;
use miner::{delay, block::write_genesis};
use ol_types::config::AppCfg;
use abscissa_core::{Command, Options, Runnable};
use std::{path::PathBuf};
use ol_types::account;
use libra_types::account_address::AccountAddress;

/// `user-wizard` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct UserWizardCmd {
    #[options(help = "path to write account manifest")]
    home_path: Option<PathBuf>,
    #[options(help = "path to file to be checked")]
    check: bool,
    #[options(help = "regenerates account manifest from mnemonic")]
    fix: bool,
    #[options(help = "creates a validator account")]
    validator: bool,
    #[options(help = "use an existing block_0.json file and skip mining")]
    block_zero: Option<PathBuf>,
}

impl Runnable for UserWizardCmd {
    /// Print version message
    fn run(&self) {
        // let miner_configs = app_config();
        let home_path = self.home_path.clone().unwrap_or_else(|| PathBuf::from("."));
        if self.check {
            match check(home_path.clone()) {
                true => println!("Proof verified in {:?}", &home_path),
                false => println!("Invalid proof in {:?}", &home_path)
            }
        } else {
            wizard(home_path, self.fix,  &self.block_zero);
        }
    }
}

pub fn wizard(path: PathBuf, is_fix: bool, block_zero: &Option<PathBuf>) -> (AccountAddress, String) {
    let mut miner_configs = AppCfg::default();
    
    let (authkey, account, _, mnemonic) = if is_fix {
        let (k, a, w) = wallet::get_account_from_prompt();
        (k, a, w, "".into())
    } else {
        wallet::keygen()
    };

    // Where to save block_0
    miner_configs.workspace.node_home = path.clone();
    miner_configs.profile.auth_key = authkey.to_string();
    miner_configs.profile.account = account;

    // Create block zero, if there isn't one.
    let block;
    if let Some(block_path) = block_zero {
        block = Block::parse_block_file(block_path.to_owned());
    } else {
        block = write_genesis(&miner_configs);
    }

    // Create Manifest
    account::UserConfigs::new(block)
    .create_manifest(path);
    (account, mnemonic)
}

/// Checks the format of the account manifest, including vdf proof
pub fn check(path: PathBuf) -> bool {
    let user_data = account::UserConfigs::get_init_data(&path).expect(&format!("could not parse manifest in {:?}", &path));

    delay::verify(&user_data.block_zero.preimage, &user_data.block_zero.proof)
}