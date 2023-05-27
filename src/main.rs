use anvil::{eth::EthApi, spawn, NodeConfig};
use anvil_benchmarks::bindings::convex::ShutdownSystemCall;
use ethers::{abi::AbiEncode, prelude::*};
use std::{env, sync::Arc, time::Instant};

#[tokio::main]
async fn main() {
    const NUM_ITERATIONS: usize = 10;

    let mut durations_rpc = Vec::new();
    let mut durations_ipc = Vec::new();
    let mut durations_reth_middleware = Vec::new();

    for _ in 0..NUM_ITERATIONS {
        let (provider, api) = spawn_http().await;
        let duration = system_shutdown(provider.clone(), &api).await;
        durations_rpc.push(duration);
    }

    for _ in 0..NUM_ITERATIONS {
        let (provider, api) = spawn_ipc().await;
        let duration = system_shutdown(provider.clone(), &api).await;
        durations_ipc.push(duration);
    }

    for _ in 0..NUM_ITERATIONS {
        let (provider, api) = spawn_reth_middleware().await;
        let duration = system_shutdown(provider.clone(), &api).await;
        durations_reth_middleware.push(duration);
    }

    let avg_duration_rpc = average_duration(&durations_rpc);
    let avg_duration_ipc = average_duration(&durations_ipc);
    let avg_duration_reth_middleware = average_duration(&durations_reth_middleware);

    println!("Average eth_call duration via http fork: {:?}", avg_duration_rpc);
    println!("Average eth_call duration via Ipc fork: {:?}", avg_duration_ipc);
    println!("Average eth_call duration via ethers-reth fork: {:?}", avg_duration_reth_middleware);
}

fn average_duration(durations: &[std::time::Duration]) -> std::time::Duration {
    let sum: std::time::Duration = durations.iter().sum();
    sum / (durations.len() as u32)
}

async fn spawn_http() -> (Arc<Provider<Ipc>>, EthApi) {
    let config = NodeConfig::default()
        .with_eth_rpc_url(Some(env::var("ETH_RPC_URL").expect("ETH_RPC_URL not found in .env")))
        .with_port(1299)
        .with_fork_block_number::<u64>(Some(14445961))
        .with_ipc(Some(None))
        .with_steps_tracing(true)
        .with_tracing(true)
        .with_gas_limit(Some(30000000000_u64))
        .no_storage_caching();

    // Spawn the node with the custom config
    let (api, handle) = spawn(config).await;

    api.anvil_auto_impersonate_account(true).await.unwrap();
    let provider =
        Arc::new(Provider::<Ipc>::connect_ipc(handle.ipc_path().unwrap()).await.unwrap());
    println!("ipc: {:?}", handle.ipc_path().unwrap());
    (provider, api)
}

async fn spawn_ipc() -> (Arc<Provider<Ipc>>, EthApi) {
    let config = NodeConfig::default()
        .with_eth_ipc_path(Some(env::var("ETH_IPC_PATH").expect("ETH_IPC_PATH not found in .env")))
        .with_fork_block_number::<u64>(Some(14445961))
        .with_ipc(Some(None))
        .with_steps_tracing(true)
        .with_gas_limit(Some(30000000000_u64))
        .no_storage_caching();

    // Spawn the node with the custom config
    let (api, handle) = spawn(config).await;

    api.anvil_auto_impersonate_account(true).await.unwrap();
    let provider =
        Arc::new(Provider::<Ipc>::connect_ipc(handle.ipc_path().unwrap()).await.unwrap());
    (provider, api)
}

async fn spawn_reth_middleware() -> (Arc<Provider<Ipc>>, EthApi) {
    let config = NodeConfig::default()
        .with_eth_ipc_path(Some(env::var("ETH_IPC_PATH").expect("ETH_IPC_PATH not found in .env")))
        .with_eth_reth_db(Some(env::var("ETH_DB_PATH").expect("ETH_DB_PATH not found in .env")))
        .with_fork_block_number::<u64>(Some(14445961))
        .with_ipc(Some(None))
        .with_steps_tracing(true)
        .with_gas_limit(Some(30000000000_u64))
        .no_storage_caching();

    // Spawn the node with the custom config
    let (api, handle) = spawn(config).await;

    api.anvil_auto_impersonate_account(true).await.unwrap();
    let provider =
        Arc::new(Provider::<Ipc>::connect_ipc(handle.ipc_path().unwrap()).await.unwrap());
    (provider, api)
}

async fn system_shutdown(provider: Arc<Provider<Ipc>>, api: &EthApi) -> std::time::Duration {
    let convex_sys: H160 = "0xF403C135812408BFbE8713b5A23a04b3D48AAE31".parse().unwrap();
    let owner: H160 = "0x3cE6408F923326f81A7D7929952947748180f1E6".parse().unwrap();

    api.anvil_set_balance(owner, U256::from(1e19 as u64)).await.unwrap();

    let shutdown = ShutdownSystemCall {}.encode().into();

    let nonce = provider.get_transaction_count(owner, None).await.unwrap();
    let gas_price = provider.get_gas_price().await.unwrap();

    let tx = TransactionRequest {
        from: Some(owner),
        to: Some(convex_sys.into()),
        value: None,
        gas_price: Some(gas_price),
        nonce: Some(nonce),
        gas: Some(28_000_000u64.into()),
        data: Some(shutdown),
        chain_id: Some(1.into()),
    };

    let start = Instant::now();
    let _result = api.call(tx.into(), Some(BlockId::Number(14445961.into())), None).await.unwrap();

    start.elapsed()
}
