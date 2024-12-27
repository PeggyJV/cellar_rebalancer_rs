use crate::application::APP;
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use signatory::FsKeyStore;
use std::path::Path;

#[derive(Command, Debug, Default, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nDelete a Cosmos Key.\n This command deletes a Cosmos key from your keystore when provided with the keyname."
)]
pub struct DeleteCosmosKeyCmd {
    /// Cosmos keyname in keystore.
    pub name: String,
}

/// The `steward keys cosmos delete [name] ` subcommand: delete the given key
impl Runnable for DeleteCosmosKeyCmd {
    fn run(&self) {
        let config = APP.config();
        let keystore = Path::new(&config.keystore);
        let keystore = signatory::FsKeyStore::open(keystore).expect("Could not open keystore");
        let name = self.name.parse().expect("Could not parse name");
        FsKeyStore::delete(&keystore, &name).unwrap();
    }
}
