mod bindings;

use anvil::{eth::EthApi, spawn, NodeConfig, NodeHandle};
use bindings::convex::ShutdownSystemCall;
use ethers::{abi::AbiEncode, prelude::*};
use lazy_static::lazy_static;
use ndarray::Array1;
use ndarray_stats::QuantileExt;
use std::{
    env,
    future::Future,
    sync::{Arc, Mutex},
    time::Instant,
};

lazy_static! {
    static ref HTTP_FLAG: Mutex<bool> = Mutex::new(true);
    static ref IPC_FLAG: Mutex<bool> = Mutex::new(true);
    static ref RETH_FLAG: Mutex<bool> = Mutex::new(true);
}

//TODO: fix the num iteration limitation (to many connections) & fix the trace subscriber creation for the reruns of the ipc

#[tokio::main]
async fn main() {
    const NUM_ITERATIONS: usize = 5;

    let durations_rpc = collect_durations(NUM_ITERATIONS, spawn_http).await;
    print_statistics("http fork", &durations_rpc);

    let durations_ipc = collect_durations(NUM_ITERATIONS, spawn_ipc).await;
    print_statistics("Ipc fork", &durations_ipc);

    //let durations_ethers_reth = collect_durations(NUM_ITERATIONS, spawn_ethers_reth).await;
    //print_statistics("Ipc fork", &durations_ethers_reth);
}

async fn collect_durations<F, Fut>(num_iterations: usize, spawn_function: F) -> Vec<f64>
where
    F: Fn() -> Fut,
    Fut: Future<Output = (Arc<Provider<Ipc>>, EthApi, NodeHandle)> + 'static,
{
    let mut durations = vec![];
    for _ in 0..num_iterations {
        let duration = measure_system_shutdown(&spawn_function).await;
        durations.push(duration);
    }
    durations
}

async fn measure_system_shutdown<Fut>(spawn_function: impl Fn() -> Fut) -> f64
where
    Fut: Future<Output = (Arc<Provider<Ipc>>, EthApi, NodeHandle)> + 'static,
{
    let start = Instant::now();
    let (provider, api, handle) = (spawn_function)().await;
    system_shutdown(provider.clone(), &api).await;
    let duration = start.elapsed();
    shutdown(api, handle).await;
    duration.as_secs_f64()
}

fn print_statistics(label: &str, durations: &Vec<f64>) {
    let array_durations: Array1<f64> = Array1::from(durations.clone());

    let mean_duration = array_durations.mean().unwrap();
    let min_duration = array_durations.min().unwrap();
    let max_duration = array_durations.max().unwrap();

    let sum: f64 = durations.iter().map(|&x| (x - mean_duration).powi(2)).sum();
    let std_dev_duration = (sum / (durations.len() - 1) as f64).sqrt();

    println!("Mean eth_call duration via {}: {} seconds", label, mean_duration);
    println!("Std Dev of eth_call duration via {}: {} seconds", label, std_dev_duration);
    println!("Min eth_call duration via {}: {} seconds", label, min_duration);
    println!("Max eth_call duration via {}: {} seconds", label, max_duration);
}

async fn system_shutdown(provider: Arc<Provider<Ipc>>, api: &EthApi) {
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

    let _result = api.call(tx.into(), Some(BlockId::Number(14445961.into())), None).await.unwrap();
}

pub async fn shutdown(api: EthApi, handle: NodeHandle) {
    // If fork exists, flush the cache
    if let Some(fork) = api.get_fork().clone() {
        fork.database().read().await.flush_cache();
    }
    handle.server.abort();

    // shutdown the node_service, this would typically stop the service and clean up resources
    handle.node_service.abort();

    drop(api);

    // Drop the handle to fire the shutdown signal
    drop(handle);
}

async fn spawn_http() -> (Arc<Provider<Ipc>>, EthApi, NodeHandle) {
    let mut is_first_call = HTTP_FLAG.lock().unwrap();
    let config = if *is_first_call {
        *is_first_call = false;
        NodeConfig::default()
            .with_eth_rpc_url(Some(env::var("ETH_RPC_URL").expect("ETH_RPC_URL not found in .env")))
            .with_port(1299)
            .with_fork_block_number::<u64>(Some(14445961))
            .with_ipc(Some(None))
            .with_tracing(true)
            .with_steps_tracing(true)
            .with_gas_limit(Some(30000000000_u64))
            .no_storage_caching()
    } else {
        NodeConfig::default()
            .with_eth_rpc_url(Some(env::var("ETH_RPC_URL_LOCAL").expect("ETH_RPC_URL not found in .env")))
            .with_port(1299)
            .with_fork_block_number::<u64>(Some(14445961))
            .with_ipc(Some(None))
            .with_tracing(false)
            .with_steps_tracing(false)
            .with_gas_limit(Some(30000000000_u64))
            .no_storage_caching()
            .silent()
    };

    // Spawn the node with the custom config
    let (api, handle) = spawn(config).await;

    api.anvil_auto_impersonate_account(true).await.unwrap();
    let provider: Arc<Provider<Ipc>> =
        Arc::new(Provider::<Ipc>::connect_ipc(&handle.ipc_path().unwrap()).await.unwrap());
    println!("ipc: {:?}", &handle.ipc_path().unwrap());
    (provider, api, handle)
}

async fn spawn_ipc() -> (Arc<Provider<Ipc>>, EthApi, NodeHandle) {
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
    (provider, api, handle)
}

async fn spawn_ethers_reth() -> (Arc<Provider<Ipc>>, EthApi, NodeHandle) {
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
    (provider, api, handle)
}
