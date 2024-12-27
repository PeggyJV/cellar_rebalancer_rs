use crate::application::APP;
use abscissa_core::{clap::Parser, status_err, Application, Command, Runnable};
use clarity::Uint256;
use deep_space::coin::Coin;
use ethers::types::Address as EthAddress;
use gravity_bridge::cosmos_gravity::send::send_to_eth;
use gravity_bridge::gravity_proto::gravity::DenomToErc20Request;
use gravity_bridge::gravity_utils::connection_prep::{check_for_fee_denom, create_rpc_connections};
use std::{process::exit, time::Duration};

const TIMEOUT: Duration = Duration::from_secs(60);

/// Send Cosmos to the Eth chain.
#[derive(Command, Debug, Default, Parser)]
#[clap(
    long_about = "DESCRIPTION\n\nSend Cosmos token to Eth chain.\n This command sends Cosmos token to the Eth chain via the Gravity bridge. \n This command takes the Gravity denom, tx amount, Cosmos keyname, Eth destination, number of times \n transaction should be made and if the transaction should be made immediately or wait for the next \n batch."
)]
pub struct CosmosToEthCmd {
    /// Gravity denom
    #[clap(short, long)]
    denom: String,

    /// Tx amount.
    #[clap(short, long)]
    amount: String,

    /// Cosmos keyname.
    #[clap(short, long)]
    cosmos_key: String,

    /// Ethereum address
    #[clap(short, long)]
    eth_dest: String,

    /// The number of times transactions should repeat itself, default is 1.
    #[clap(short, long, default_value = "1")]
    times: String,
}

pub fn one_eth() -> f64 {
    1000000000000000000f64
}

pub fn one_atom() -> f64 {
    1000000f64
}

pub fn print_atom(input: Uint256) -> String {
    let float: f64 = input.to_string().parse().unwrap();
    let res = float / one_atom();
    format!("{}", res)
}

pub fn print_eth(input: Uint256) -> String {
    let float: f64 = input.to_string().parse().unwrap();
    let res = float / one_eth();
    format!("{}", res)
}

impl Runnable for CosmosToEthCmd {
    fn run(&self) {
        let config = APP.config();
        let denom = self.denom.to_string();
        let is_cosmos_originated = !denom.starts_with("gravity");

        let amount: Uint256 = self.amount.parse().expect("cannot parse amount");
        let cosmos_key = config
            .load_gravity_deep_space_key(self.cosmos_key.to_string())
            .expect("cannot load cosmos key");

        let cosmos_prefix = config.cosmos.prefix.trim();
        let cosmos_address = cosmos_key.to_address(cosmos_prefix).unwrap();
        let cosmos_grpc = config.cosmos.grpc.trim();
        println!("Sending from Cosmos address {}", cosmos_address);
        abscissa_tokio::run_with_actix(&APP, async {
        let connections = create_rpc_connections(
            cosmos_prefix.to_string(),
            Some(cosmos_grpc.to_string()),
            None,
            TIMEOUT,
        )
        .await;
        let contact = connections.contact.unwrap();
        let mut grpc = connections.grpc.unwrap();
        let res = grpc
            .denom_to_erc20(DenomToErc20Request {
                denom: denom.clone(),
            })
            .await;
        match res {
            Ok(val) => println!(
                "Asset {} has ERC20 representation {}",
                denom,
                val.into_inner().erc20
            ), Err(_e) => {
                println!(
                    "Asset {} has no ERC20 representation, you may need to deploy an ERC20 for it!",
                    denom
                );
                exit(1);
            }
        }

        let amount = Coin {
            amount: amount.clone(),
            denom: denom.clone(),
        };
        let bridge_fee = Coin {
            amount: 1u64.into(),
            denom: denom.clone(),
        };

        let eth_dest: EthAddress = self.eth_dest.parse().expect("cannot parse ethereum address");
        check_for_fee_denom(&denom, cosmos_address, &contact).await;

        let balances = contact
            .get_balances(cosmos_address)
            .await
            .expect("Failed to get balances!");
        let mut found = None;
        for coin in balances.iter() {
            if coin.denom == denom {
                found = Some(coin);
            }
        }

        println!("Cosmos balances {:?}", balances);
        let times = self.times.parse::<usize>().expect("cannot parse times");

        match found {
            None => panic!("You don't have any {} tokens!", denom),
            Some(found) => {
                if amount.amount.clone() * times.into() >= found.amount && times == 1 {
                    if is_cosmos_originated {
                        panic!("Your transfer of {} {} tokens is greater than your balance of {} tokens. Remember you need some to pay for fees!", print_atom(amount.amount), denom, print_atom(found.amount.clone()));
                    } else {
                        panic!("Your transfer of {} {} tokens is greater than your balance of {} tokens. Remember you need some to pay for fees!", print_eth(amount.amount), denom, print_eth(found.amount.clone()));
                    }
                } else if amount.amount.clone() * times.into() >= found.amount {
                    if is_cosmos_originated {
                        panic!("Your transfer of {} * {} {} tokens is greater than your balance of {} tokens. Try to reduce the amount or the --times parameter", print_atom(amount.amount), times, denom, print_atom(found.amount.clone()));
                    } else {
                        panic!("Your transfer of {} * {} {} tokens is greater than your balance of {} tokens. Try to reduce the amount or the --times parameter", print_eth(amount.amount), times, denom, print_eth(found.amount.clone()));
                    }
                }
            }
        }

        for _ in 0..times {
            println!(
                "Locking {} / {} into the batch pool",
                amount.clone(),
                denom
            );
            let res = send_to_eth(
                cosmos_key,
                eth_dest,
                amount.clone(),
                bridge_fee.clone(),
                config.cosmos.gas_price.as_tuple(),
                &contact,
                1.0
            )
            .await;
            match res {
                Ok(tx_id) => println!("Send to Eth txid {}", tx_id.txhash),
                Err(e) => println!("Failed to send tokens! {:?}", e),
            }
        }

        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
        });
    }
}
