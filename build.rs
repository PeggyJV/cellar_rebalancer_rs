#![allow(clippy::all)]
use ethers::contract::Abigen;
use std::process;

use std::path::Path;
use std::{
    fs::{self, create_dir_all, remove_dir_all},
    path::PathBuf,
};

use merkle_hash::{Algorithm, MerkleTree};
use walkdir::WalkDir;

/// A temporary directory for proto building
const TMP_PATH: &str = "/tmp/steward/";
/// the output directory
const OUT_PATH: &str = "crates/steward-proto/src/gen/";
const HASH_ABI_FILE: &str = "hash_abi";
const HASH_PROTO_FILE: &str = "hash_proto";

fn main() {
    // only generate bindings if changes have occurred
    let previous_abi_hash = fs::read_to_string(HASH_ABI_FILE).unwrap_or_default();
    let previous_proto_hash = fs::read_to_string(HASH_PROTO_FILE).unwrap_or_default();
    let current_abi_hash = MerkleTree::builder("abi")
        .algorithm(Algorithm::Blake3)
        .hash_names(false)
        .build()
        .expect("abi dir merkle tree build failed")
        .root
        .item
        .hash;
    let current_proto_hash = MerkleTree::builder("proto")
        .algorithm(Algorithm::Blake3)
        .hash_names(false)
        .build()
        .expect("proto dir merkle tree build failed")
        .root
        .item
        .hash;
    let current_abi_hash = hex::encode(current_abi_hash);
    let current_proto_hash = hex::encode(current_proto_hash);

    if current_abi_hash != previous_abi_hash {
        generate_contract_abis();
        fs::write(HASH_ABI_FILE, current_abi_hash).expect("failed to write abi hash");
    }

    if current_proto_hash != previous_proto_hash {
        generate_rust_protos();
        fs::write(HASH_PROTO_FILE, current_proto_hash).expect("failed to write proto hash");
    }
}

fn generate_contract_abis() {
    // (JSON/Contract name, output file name)
    let contracts = vec![
        ("AaveV2StablecoinCellar", "aave_v2_stablecoin"),
        ("CellarV1", "cellar_v1"),
        ("CellarV2", "cellar_v2"),
        ("CellarV2_2", "cellar_v2_2"),
        ("UniswapV3AdaptorV1", "uniswap_v3_adaptor_v1"),
        ("UniswapV3AdaptorV2", "uniswap_v3_adaptor_v2"),
        ("AaveATokenAdaptorV1", "aave_a_token_adaptor_v1"),
        ("AaveDebtTokenAdaptorV1", "aave_debt_token_adaptor_v1"),
        ("AaveATokenAdaptorV2", "aave_a_token_adaptor_v2"),
        ("AaveDebtTokenAdaptorV2", "aave_debt_token_adaptor_v2"),
        ("AaveV3ATokenAdaptorV1", "aave_v3_a_token_adaptor_v1"),
        ("AaveV3DebtTokenAdaptorV1", "aave_v3_debt_token_adaptor_v1"),
        ("CellarAdaptorV1", "cellar_adaptor_v1"),
        ("CompoundCTokenAdaptorV2", "compound_c_token_adaptor_v2"),
        ("OneInchAdaptorV1", "oneinch_adaptor_v1"),
        ("ZeroXAdaptorV1", "zero_x_adaptor_v1"),
        ("SwapWithUniswapAdaptorV1", "swap_with_uniswap_adaptor_v1"),
        ("FeesAndReservesAdaptorV1", "fees_and_reserves_adaptor_v1"),
        ("VestingSimpleAdaptorV2", "vesting_simple_adaptor_v2"),
        (
            "AaveV2EnableAssetAsCollateralAdaptorV1",
            "aave_v2_enable_asset_as_collateral_adaptor_v1",
        ),
        ("FTokenAdaptor", "f_token_adaptor"),
        (
            "MorphoAaveV2ATokenAdaptorV1",
            "morpho_aave_v2_a_token_adaptor_v1",
        ),
        (
            "MorphoAaveV2DebtTokenAdaptorV1",
            "morpho_aave_v2_debt_token_adaptor_v1",
        ),
        (
            "MorphoAaveV3ATokenCollateralAdaptorV1",
            "morpho_aave_v3_a_token_collateral_adaptor_v1",
        ),
        (
            "MorphoAaveV3ATokenP2PAdaptorV1",
            "morpho_aave_v3_a_token_p2p_adaptor_v1",
        ),
        (
            "MorphoAaveV3DebtTokenAdaptorV1",
            "morpho_aave_v3_debt_token_adaptor_v1",
        ),
        ("MorphoRewardHandler", "morpho_reward_handler"),
        ("BalancerPoolAdaptorV1", "balancer_pool_adaptor_v1"),
        ("CellarV2_5", "cellar_v2_5"),
        (
            "CellarWithShareLockPeriodV1",
            "cellar_with_share_lock_period_v1",
        ),
        ("DebtFTokenAdaptorV1", "debt_f_token_adaptor_v1"),
        ("CollateralFTokenAdaptorV1", "collateral_f_token_adaptor_v1"),
        ("LegacyCellarAdaptorV1", "legacy_cellar_adaptor_v1"),
        ("CurveAdaptorV1", "curve_adaptor_v1"),
        ("ConvexCurveAdaptorV1", "convex_curve_adaptor_v1"),
        ("AuraERC4626AdaptorV1", "aura_erc4626_adaptor_v1"),
        (
            "CellarWithMultiAssetDepositV1",
            "cellar_with_multi_asset_deposit_v1",
        ),
        ("MorphoBlueSupplyAdaptorV1", "morpho_blue_supply_adaptor_v1"),
        (
            "MorphoBlueCollateralAdaptorV1",
            "morpho_blue_collateral_adaptor_v1",
        ),
        ("MorphoBlueDebtAdaptorV1", "morpho_blue_debt_adaptor_v1"),
        ("ERC4626AdaptorV1", "erc4626_adaptor_v1"),
        ("StakingAdaptorV1", "staking_adaptor_v1"),
        ("PendleAdaptorV1", "pendle_adaptor_v1"),
        (
            "boring-vault/ManagerWithMerkleVerification",
            "boring_vault/manager_with_merkle_verification",
        ),
    ];

    contracts.iter().for_each(|n| {
        let json_file_name = n.0;
        let rust_file_name = n.1;
        let identifier = if json_file_name.contains('/') {
            json_file_name.split('/').last().unwrap()
        } else {
            json_file_name
        };
        let abigen = match Abigen::new(identifier, format!("abi/{}.json", json_file_name)) {
            Ok(abigen) => abigen,
            Err(e) => {
                println!("Could not open {}.json: {}", json_file_name, e);
                process::exit(1);
            }
        };

        let abi = match abigen
            .add_event_derive("serde::Deserialize")
            .add_event_derive("serde::Serialize")
            .generate()
        {
            Ok(abi) => abi,
            Err(e) => {
                println!("Could not generate abi from {}.json: {}", json_file_name, e);
                process::exit(1);
            }
        };

        match abi.write_to_file(format!("src/gen/abi/{}.rs", rust_file_name)) {
            Ok(_) => (),
            Err(e) => println!("Error writing {}.rs: {}", rust_file_name, e),
        }
    })
}

fn generate_rust_protos() {
    let out_dir = Path::new(&OUT_PATH);
    let tmp_dir = Path::new(&TMP_PATH);
    let root = env!("CARGO_MANIFEST_DIR");
    let root: PathBuf = root.parse().unwrap();
    let mut steward_proto_dir = root.clone();
    steward_proto_dir.push("proto/steward/v4");
    let mut boring_vault_proto_dir = root;
    boring_vault_proto_dir.push("proto/steward/v4/boring_vault/v1");
    let steward_proto_dir = [steward_proto_dir, boring_vault_proto_dir];

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    for proto_path in &steward_proto_dir {
        protos.append(
            &mut WalkDir::new(proto_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().is_some()
                        && e.path().extension().unwrap() == "proto"
                })
                .map(|e| e.into_path())
                .collect(),
        );
    }

    // create directories for temporary build dirs
    fs::create_dir_all(tmp_dir)
        .unwrap_or_else(|_| panic!("Failed to create {:?}", tmp_dir.to_str()));

    // Compile all proto files
    let mut config = prost_build::Config::default();
    config.out_dir(tmp_dir);
    config.type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]");
    config.compile_protos(&protos, &steward_proto_dir).unwrap();

    // Compile all proto client for GRPC services
    println!("Compiling proto clients for GRPC services...");
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .file_descriptor_set_path(tmp_dir.join("descriptor.bin"))
        .format(true)
        .out_dir(tmp_dir)
        .compile_with_config(config, &protos, &steward_proto_dir)
        .unwrap();

    copy_generated_files(tmp_dir, out_dir);
    println!("Done!");
}

fn copy_generated_files(from_dir: &Path, to_dir: &Path) {
    println!("Copying generated files into '{}'...", to_dir.display());

    // Remove old compiled files
    remove_dir_all(&to_dir).unwrap_or_default();
    create_dir_all(&to_dir).unwrap();

    let mut filenames = Vec::new();

    // Copy new compiled files (prost does not use folder structures)
    let errors = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();
            filenames.push(filename.clone());
            let contents = fs::read(e.path())?;
            fs::write(format!("{}/{}", to_dir.display(), &filename), contents)
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            eprintln!("[error] Error while copying compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }
}
