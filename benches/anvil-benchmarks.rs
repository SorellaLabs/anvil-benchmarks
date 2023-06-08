use criterion::{criterion_group, criterion_main, Criterion};
use std::error::Error;

mod bindings;
use bindings::convex::ShutdownSystemCall;

use anvil::{eth::EthApi, spawn, NodeConfig};
use ethers::{abi::AbiEncode, prelude::*};
use std::{env, sync::Arc};
use tokio::runtime::Runtime;

use std::str::FromStr;

const GAS: u64 = 28_000_000;
struct SpawnResult {
    api: EthApi,
    transaction: TransactionRequest,
}

impl SpawnResult {
    fn new(api: EthApi, transaction: TransactionRequest) -> Self {
        Self { api, transaction }
    }
}

pub fn benchmark_http_local(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let anvil_result = rt.block_on(async { spawn_http_local().await.unwrap() });

    c.bench_function("http_local_shutdown", |b| {
        b.to_async(&rt).iter(|| async {
            system_shutdown(&anvil_result.api, anvil_result.transaction.clone()).await
        })
    });
}

pub fn benchmark_http(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let anvil_result = rt.block_on(async { spawn_http_external().await.unwrap() });

    c.bench_function("http_shutdown", |b: &mut criterion::Bencher<'_>| {
        b.to_async(&rt).iter(|| async {
            system_shutdown(&anvil_result.api, anvil_result.transaction.clone()).await
        })
    });
}

pub fn benchmark_ipc(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let anvil_result = rt.block_on(async { spawn_ipc().await.unwrap() });

    c.bench_function("ipc_shutdown", |b| {
        b.to_async(&rt).iter(|| async {
            system_shutdown(&anvil_result.api, anvil_result.transaction.clone()).await
        })
    });
}

pub fn benchmark_reth(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let anvil_result = rt.block_on(async { spawn_ethers_reth().await.unwrap() });

    c.bench_function("reth_shutdown", |b| {
        b.to_async(&rt).iter(|| async {
            system_shutdown(&anvil_result.api, anvil_result.transaction.clone()).await
        })
    });
}

async fn system_shutdown(api: &EthApi, transaction: TransactionRequest) {
    api.call(transaction.into(), Some(BlockId::Number(14445961.into())), None).await.unwrap();
}

async fn spawn_with_config(config: NodeConfig) -> Result<SpawnResult, Box<dyn Error>> {
    let (api, handle) = spawn(config).await;

    api.anvil_auto_impersonate_account(true).await?;

    let convex_sys = H160::from_str("0xF403C135812408BFbE8713b5A23a04b3D48AAE31").unwrap();
    let owner = H160::from_str("0x3cE6408F923326f81A7D7929952947748180aF46").unwrap();
    let data: Bytes = ShutdownSystemCall {}.encode().into();

    let transaction =
        TransactionRequest::new().from(owner).to(convex_sys).gas::<U256>(GAS.into()).data(data);

    Ok(SpawnResult::new(api, transaction))
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
        .with_steps_tracing(false)
        .with_gas_limit(Some(GAS))
        .no_storage_caching()
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
        .with_steps_tracing(false)
        .with_gas_limit(Some(GAS))
        .no_storage_caching()
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
        .with_gas_limit(Some(GAS))
        .no_storage_caching()
        .silent()
        .with_steps_tracing(false);

    spawn_with_config(config).await
}

criterion_group!(benches, benchmark_http_local, benchmark_http, benchmark_ipc, benchmark_reth);
criterion_main!(benches);
