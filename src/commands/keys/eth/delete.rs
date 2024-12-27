use crate::application::APP;
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use signatory::FsKeyStore;
use std::path;

#[derive(Command, Debug, Default, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nDelete an Eth Key.\n This command deletes an Eth key from your keystore when provided with the keyname."
)]
pub struct DeleteKeyCmd {
    /// Eth keyname in keystore.
    pub name: String,
}

impl Runnable for DeleteKeyCmd {
    fn run(&self) {
        let config = APP.config();
        let keystore = path::Path::new(&config.keystore);
        let keystore = FsKeyStore::open(keystore).expect("Could not open keystore");

        let name = self.name.parse().expect("Could not parse name");
        keystore.delete(&name).expect("Could not delete key");
    }
}
