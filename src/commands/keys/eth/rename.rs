use crate::application::APP;
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use signatory::FsKeyStore;
use std::path;

#[derive(Command, Debug, Default, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nRename an Eth key.\n This command will rename an Eth key in the keystore. It takes the existing keyname and new keyname."
)]
pub struct RenameKeyCmd {
    /// Eth keyname in keystore.
    pub name: String,

    /// New keyname to replace name in keystore.
    pub new_name: String,

    /// Overwrite key with the same name in the keystore when set to true. Takes a Boolean.
    #[clap(short, long)]
    pub overwrite: bool,
}

impl Runnable for RenameKeyCmd {
    fn run(&self) {
        let config = APP.config();
        let keystore = path::Path::new(&config.keystore);
        let keystore = FsKeyStore::open(keystore).expect("Could not open keystore");

        let name = self.name.parse().expect("Could not parse name");
        let new_name = self.new_name.parse().expect("Could not parse new_name");
        if let Ok(_info) = keystore.info(&new_name) {
            if !self.overwrite {
                println!("Key already exists, exiting.");
                return;
            }
        }

        let key = keystore.load(&name).expect("Could not load key");
        keystore.store(&new_name, &key).unwrap();
        keystore.delete(&name).unwrap();
    }
}
