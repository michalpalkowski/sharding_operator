use clap::Parser;
use operator::events::look_for_event;
use starknet::{core::utils::starknet_keccak, providers::Url};
use starknet_crypto::Felt;

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
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let rpc: Url = args.rpc_url.parse().unwrap();
    let initial_event_hash = starknet_keccak("ShardInitialized".as_bytes());
    let end_event_hash = starknet_keccak(args.event_name.as_bytes());

    if let Some((_initial_event, initial_block)) = look_for_event(args.proxy_address, args.start_block, rpc.clone(), initial_event_hash).await {
        let _end_event = look_for_event(args.contract_address, initial_block, rpc.clone(), end_event_hash).await;
    }
}
