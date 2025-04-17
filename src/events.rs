use starknet::{
    core::types::{BlockId, Event},
    providers::{
        JsonRpcClient, Provider, ProviderError::StarknetError, Url, jsonrpc::HttpTransport,
    },
};
use starknet_crypto::Felt;
use std::sync::Arc;
use tokio::time::sleep;

pub async fn look_for_event(
    contract_address: Felt,
    start_block: u64,
    rpc_url: Url,
    event_hash: Felt,
) -> Option<(Event, u64)> {
    let provider: Arc<JsonRpcClient<HttpTransport>> =
        Arc::new(JsonRpcClient::new(HttpTransport::new(rpc_url)));
    let mut current_block = start_block;
    loop {
        let events = get_events_for_blocks(current_block, provider.clone()).await;
        for event in events {
            if event.from_address == contract_address && event.keys[0] == event_hash {
                println!("Found event: {:?}", event);
                return Some((event, current_block));
            }
        }
        sleep(std::time::Duration::from_secs(1)).await;
        println!("Looking for event in block: {}", current_block);
        current_block += 1;
    }
}

pub async fn get_events_for_blocks(
    block_number: u64,
    provider: Arc<JsonRpcClient<HttpTransport>>,
) -> Vec<Event> {
    let block = BlockId::Number(block_number);

    loop {
        match provider.get_block_with_receipts(block).await {
            Ok(block_with_receipts) => {
                let events = block_with_receipts
                    .transactions()
                    .iter()
                    .flat_map(|tx| tx.receipt.events().to_vec())
                    .collect::<Vec<Event>>();
                return events;
            }
            Err(StarknetError(starknet::core::types::StarknetError::BlockNotFound)) => {
                println!("Block {} not found yet. Retrying...", block_number);
                sleep(std::time::Duration::from_secs(2)).await;
            }
            Err(e) => {
                println!("Error fetching block {}: {:?}", block_number, e);
                return vec![];
            }
        }
    }
}

#[tokio::test]
async fn test_look_for_event() {
    use starknet::core::utils::starknet_keccak;

    pub const RPC_URL: &str = "https://starknet-sepolia.public.blastapi.io";
    let address = Felt::from_hex_unchecked(
        "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
    );
    let event_hash = starknet_keccak("Transfer".as_bytes());
    println!("Event hash: {:?}", event_hash);
    let start_block = 612520;
    let rpc_url = Url::parse(RPC_URL).unwrap();
    look_for_event(address, start_block, rpc_url, event_hash).await;
}
