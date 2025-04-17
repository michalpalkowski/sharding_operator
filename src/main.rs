use clap::Parser;
use operator::events::look_for_event;
use starknet::{core::utils::starknet_keccak, providers::Url};
use starknet_crypto::Felt;
// use katana::run as run_katana;
use saya::Sharding;
use std::path::PathBuf;
#[derive(Parser)]
struct Cli {
    #[arg(env, short, long)]
    contract_address: Felt,
    #[arg(env, short, long)]
    proxy_address: Felt,
    #[arg(env, short, long)]
    start_block: u64,
    #[arg(env, short, long)]
    rpc_url: String,
    #[arg(env, short, long)]
    event_name: String,
    #[arg(env, short, long)]
    atlantic_key: String,
    #[arg(env, short, long)]
    account_address: Felt,
    #[arg(env, short, long)]
    account_private_key: Felt,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let rpc: Url = args.rpc_url.parse().unwrap();
    let initial_event_hash = starknet_keccak("ShardInitialized".as_bytes());

    if let Some((_initial_event, initial_block)) = look_for_event(
        args.proxy_address,
        args.start_block,
        rpc.clone(),
        initial_event_hash,
    )
    .await
    {
        println!("Initial event found at block {}", initial_block);

        // let katana_config = KatanaConfig {
        //     chain: "sharding",
        //     block_time: 5000,
        //     db_dir: "katana.db",
        // };

        // if let Err(e) = run_katana(katana_config).await {
        //     eprintln!("Failed to start katana: {}", e);
        //     return;
        // }

        let saya = Sharding {
            rollup_rpc: rpc.clone(),
            snos_program: PathBuf::from("../saya/programs/snos.json"),
            db_dir: None,
            atlantic_key: args.atlantic_key,
            mock_snos_from_pie: true,
            shard_contract_address: args.proxy_address,
            game_contract_address: args.contract_address,
            event_name: args.event_name,
            account_address: args.account_address,
            account_private_key: args.account_private_key,
        };

        if let Err(e) = saya.run().await {
            eprintln!("Failed to run saya: {}", e);
            return;
        }
    }
}
