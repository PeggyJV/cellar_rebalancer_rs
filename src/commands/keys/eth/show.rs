use crate::application::APP;
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use ethers::prelude::*;
use signatory::FsKeyStore;
use std::path;

#[derive(Command, Debug, Default, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nShow details of an Eth key in the keystore.\n This command shows details of an Eth key in the keystore, it takes the name of the key."
)]
pub struct ShowKeyCmd {
    /// Eth keyname
    pub name: String,
}

// Entry point for `contract monitor keys show [name]`
impl Runnable for ShowKeyCmd {
    fn run(&self) {
        let config = APP.config();
        let keystore = path::Path::new(&config.keystore);
        let keystore = FsKeyStore::open(keystore).expect("Could not open keystore");
        let name = self.name.parse().expect("Could not parse name");

        let key = keystore.load(&name).expect("Could not load key");

        let key = key
            .to_pem()
            .parse::<k256::elliptic_curve::SecretKey<k256::Secp256k1>>()
            .expect("Could not parse key");

        let wallet: LocalWallet = Wallet::from(key);

        let address = wallet.address();

        println!("{}\t{:#x}", name, address);
    }
}
