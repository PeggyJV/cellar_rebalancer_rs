use crate::config::StewardConfig;
use crate::{application::APP, prelude::*};
use abscissa_core::{Command, Clap, Runnable};

#[derive(Command, Debug, Default, Clap)]
pub struct PrintConfigCmd {
    #[clap(short, long)]
    show_default: bool,
}

impl Runnable for PrintConfigCmd {
    fn run(&self) {
        let config = if self.show_default {
            StewardConfig::default()
        } else {
            let config = APP.config();
            StewardConfig {
                cellars: config.cellars.to_owned(),
                ethereum: config.ethereum.to_owned(),
                keys: config.keys.to_owned(),
                keystore: config.keys.keystore.to_owned(),
                gravity: config.gravity.to_owned(),
                cosmos: config.cosmos.to_owned(),
                metrics: config.metrics.to_owned(),
            }
        };

        print!("{}", toml::to_string(&config).unwrap());
    }
}
