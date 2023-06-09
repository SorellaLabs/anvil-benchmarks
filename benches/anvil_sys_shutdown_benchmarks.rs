use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use std::error::Error;

mod bindings;
use bindings::convex::ShutdownSystemCall;

use anvil::{eth::EthApi, spawn, NodeConfig};
use ethers::{abi::AbiEncode, prelude::*};
use std::{env, sync::Arc};
use tokio::{runtime::Runtime, time::Duration};

const GAS: u64 = 30_000_000;
struct SpawnResult {
    api: EthApi,
    provider: Arc<Provider<Ipc>>,
}

impl SpawnResult {
    fn new(api: EthApi, provider: Arc<Provider<Ipc>>) -> Self {
        Self { api, provider }
    }
}

async fn system_shutdown(api: &EthApi, provider: Arc<Provider<Ipc>>) {
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
    api.call(tx.into(), Some(BlockId::Number(14445961.into())), None).await.unwrap();
}

async fn spawn_with_config(config: NodeConfig) -> Result<SpawnResult, Box<dyn Error>> {
    let (api, handle) = spawn(config).await;
    let provider = Arc::new(Provider::<Ipc>::connect_ipc(handle.ipc_path().unwrap()).await?);
    Ok(SpawnResult::new(api, provider))
}
async fn spawn_http_local() -> Result<SpawnResult, Box<dyn Error>> {
    spawn_http(true).await
}

async fn spawn_http_external() -> Result<SpawnResult, Box<dyn Error>> {
    spawn_http(false).await
}

async fn spawn_ipc() -> Result<SpawnResult, Box<dyn Error>> {
    let ipc_path = env::var("ETH_IPC_PATH").expect("ETH_IPC_PATH not found in .env");
    let config = NodeConfig::default()
        .with_eth_ipc_path(Some(ipc_path.to_string()))
        .with_fork_block_number::<u64>(Some(14445961))
        .with_ipc(Some(None))
        .with_gas_limit(Some(GAS))
        .no_storage_caching()
        .with_steps_tracing(false)
        .with_tracing(false)
        .silent();

    spawn_with_config(config).await
}

async fn spawn_ethers_reth() -> Result<SpawnResult, Box<dyn Error>> {
    let ipc_path = env::var("ETH_IPC_PATH").expect("ETH_IPC_PATH not found in .env");
    let db_path = env::var("ETH_DB_PATH").expect("ETH_DB_PATH not found in .env");

    let config = NodeConfig::default()
        .with_eth_ipc_path(Some(ipc_path.to_string()))
        .with_eth_reth_db(Some(db_path.to_string()))
        .with_fork_block_number::<u64>(Some(14445961))
        .with_ipc(Some(None))
        .with_gas_limit(Some(GAS))
        .no_storage_caching()
        .with_steps_tracing(false)
        .with_tracing(false)
        .silent();

    spawn_with_config(config).await
}

async fn spawn_http(local: bool) -> Result<SpawnResult, Box<dyn Error>> {
    let rpc_url = match local {
        true => env::var("ETH_RPC_URL_LOCAL").expect("ETH_RPC_URL_LOCAL not found in .env"),
        false => env::var("ETH_RPC_URL").expect("ETH_RPC_URL not found in .env"),
    };

    let config = NodeConfig::default()
        .with_eth_rpc_url(Some(rpc_url.to_string()))
        .with_port(1299)
        .with_fork_block_number::<u64>(Some(14445961))
        .with_ipc(Some(None))
        .with_gas_limit(Some(GAS))
        .no_storage_caching()
        .with_steps_tracing(false)
        .with_tracing(false)
        .silent()
        .fork_compute_units_per_second(Some(100))
        .fork_request_retries(Some(12))
        .fork_request_timeout(Some(Duration::from_millis(50000)));

    spawn_with_config(config).await
}

pub fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("AnvilShutdown");

    // An array of functions that spawn nodes with different configurations
    let spawn_funcs: [(fn() -> SpawnResult, &str); 3] = [
        (
            || {
                let rt = Runtime::new().unwrap();
                rt.block_on(async { spawn_http_local().await.unwrap() })
            },
            "HTTP Local",
        ),
        (
            || {
                let rt = Runtime::new().unwrap();
                rt.block_on(async { spawn_ipc().await.unwrap() })
            },
            "IPC",
        ),
        (
            || {
                let rt = Runtime::new().unwrap();
                rt.block_on(async { spawn_ethers_reth().await.unwrap() })
            },
            "RETH",
        ),
    ];

    // For each spawn function...
    for (spawn_func, description) in spawn_funcs.iter() {
        // Benchmark system_shutdown
        group.bench_function(BenchmarkId::new("Shutdown", description), |b| {
            b.to_async(Runtime::new().unwrap()).iter(|| async {
                let anvil_result = spawn_func();
                let api = &anvil_result.api;
                let provider = anvil_result.provider.clone();

                system_shutdown(api, provider.clone()).await
            })
        });
    }

    group.finish();
}
criterion_group!(benches, benchmarks);
criterion_main!(benches);