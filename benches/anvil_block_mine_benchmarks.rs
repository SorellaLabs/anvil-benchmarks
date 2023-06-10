// in this bench I want to send a blocks worth of transaction & see how long it takes the node to
// mine it
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use std::error::Error;

mod bindings;
use bindings::convex::ShutdownSystemCall;

use anvil::{eth::EthApi, spawn, NodeConfig};
use ethers::{abi::AbiEncode, prelude::*};
use std::{env, pin::Pin, sync::Arc};
use tokio::{macros::support::Future, runtime::Runtime, time::Duration};

const GAS: u64 = 30_000_000;
struct SpawnResult {
    api: EthApi,
    provider: Arc<Provider<Ipc>>,
}

struct Block {
    txs: Vec<TransactionRequest>,
    relative_timestamp: Vec<Duration>,
}

pub fn get_txs(block_number: u64) -> Vec<TransactionRequest> {
    let mut txs = Vec::new();
    for i in 0..100 {
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
        txs.push(tx);
    }
    txs
}

pub fn get_block(start_block: u64, end_block: u64) -> Vec<Block> {
    let mut blocks = Vec::new();
    for block_number in start_block..end_block {
        let block = Block::new();
        let txs = get_txs(block_number);
        for tx in txs {
            block.add_tx(tx);
        }
        blocks.push(block);
    }
    blocks
}

impl Block {
    fn new() -> Self {
        Self { txs: Vec::new(), relative_timestamp: Vec::new() }
    }
    fn add_tx(&mut self, tx: TransactionRequest, relative_timestamp: Duration) {
        self.txs.push(tx);
        self.relative_timestamp.push(relative_timestamp);
    }
}

impl SpawnResult {
    fn new(api: EthApi, provider: Arc<Provider<Ipc>>) -> Self {
        Self { api, provider }
    }
}

async fn block_simulation(blocks: Vec<Block>, api: &EthApi, provider: Arc<Provider<Ipc>>) {
    for block in blocks {
        for tx in block {
            api.send_transaction(tx).await.unwrap();
        }
        api.mine_one().await;
    }
}

async fn spawn_with_config(config: NodeConfig) -> Result<SpawnResult, Box<dyn Error>> {
    let (api, handle) = spawn(config).await;
    let provider = Arc::new(Provider::<Ipc>::connect_ipc(handle.ipc_path().unwrap()).await?);
    Ok(SpawnResult::new(api, provider))
}
async fn spawn_http_local() -> Result<SpawnResult, Box<dyn Error>> {
    spawn_http(true).await
}

async fn spawn_http_remote() -> Result<SpawnResult, Box<dyn Error>> {
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
        .fork_compute_units_per_second(Some(300))
        .fork_request_timeout(Some(Duration::from_millis(1000000)));

    spawn_with_config(config).await
}

pub fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Convex system shutdown simulation using anvil");

    // An array of async functions that spawn nodes with different configurations
    let spawn_funcs: [(fn() -> Pin<Box<dyn Future<Output = SpawnResult>>>, &str); 3] = [
        (|| Box::pin(async { spawn_http_local().await.unwrap() }), "Local Http"),
        (|| Box::pin(async { spawn_ipc().await.unwrap() }), "Ipc"),
        (|| Box::pin(async { spawn_ethers_reth().await.unwrap() }), "ethers-reth middleware"),
    ];

    for (spawn_func, description) in spawn_funcs.iter() {
        let spawn_func = spawn_func.clone();

        group.sample_size(15).bench_function(BenchmarkId::new("Shutdown", description), move |b| {
            b.iter(|| {
                let rt = Runtime::new().unwrap();
                let spawn_func = spawn_func.clone();

                rt.block_on(async {
                    // Spawn a new node with the appropriate configuration
                    let anvil_result = spawn_func().await;
                    block_simulation(&anvil_result.api, anvil_result.provider.clone()).await
                })
            })
        });
    }

    // Special case for HTTP Remote due to rate limiting.
    let spawn_http_remote = || Box::pin(async { spawn_http_remote().await.unwrap() });
    group.sample_size(10).bench_function(BenchmarkId::new("Shutdown", "HTTP Remote"), move |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            let spawn_func = spawn_http_remote.clone();

            rt.block_on(async {
                // Spawn a new node with the appropriate configuration
                let anvil_result = spawn_func().await;
                system_shutdown(&anvil_result.api, anvil_result.provider.clone()).await
            })
        })
    });

    group.finish();
}
criterion_group!(benches, benchmarks);
criterion_main!(benches);
