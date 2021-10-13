use crate::application::APP;
use abscissa_core::{Application, Command, Clap, Runnable};
use signatory::FsKeyStore;
use std::path;

#[derive(Command, Debug, Default, Clap)]
pub struct DeleteKeyCmd {
    #[clap()]
    pub args: Vec<String>,
}

// Entry point for `keys delete [name]`
// - [name] required; key name
impl Runnable for DeleteKeyCmd {
    fn run(&self) {
        let config = APP.config();
        let keystore = path::Path::new(&config.keys.keystore);
        let keystore = FsKeyStore::create_or_open(keystore).expect("Could not open keystore");

        let name = self.args.get(0).expect("name is required");
        let name = name.parse().expect("Could not parse name");
        keystore.delete(&name).expect("Could not delete key");
    }
}