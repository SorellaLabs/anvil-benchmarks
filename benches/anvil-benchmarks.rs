use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput, PlotConfiguration, AxisScale};

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
    transaction: TransactionRequest,
}

impl SpawnResult {
    fn new(api: EthApi, transaction: TransactionRequest) -> Self {
        Self { api, transaction }
    }
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

pub fn benchmark_startup_ipc(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("ipc_startup", |b| {
        b.to_async(&rt).iter(|| async {
            spawn_ipc().await.unwrap();
        })
    });
}

pub fn benchmark_startup_and_shutdown_ipc(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("ipc_startup_and_shutdown", |b| {
        b.to_async(&rt).iter(|| async {
            let anvil_result = spawn_ipc().await.unwrap();
            system_shutdown(&anvil_result.api, anvil_result.transaction.clone()).await;
        })
    });
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

pub fn benchmark_startup_http_local(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("http_local_startup", |b| {
        b.to_async(&rt).iter(|| async {
            spawn_http_local().await.unwrap();
        })
    });
}

pub fn benchmark_startup_and_shutdown_http_local(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("http_local_startup_and_shutdown", |b| {
        b.to_async(&rt).iter(|| async {
            let anvil_result = spawn_http_local().await.unwrap();
            system_shutdown(&anvil_result.api, anvil_result.transaction.clone()).await;
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

    let convex_sys: H160 = "0xF403C135812408BFbE8713b5A23a04b3D48AAE31".parse().unwrap();
    let owner: H160 = "0x3cE6408F923326f81A7D7929952947748180f1E6".parse().unwrap();

    api.anvil_set_balance(owner, U256::from(1e19 as u64)).await.unwrap();
    let provider = Arc::new(Provider::<Ipc>::connect_ipc(handle.ipc_path().unwrap()).await?);
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

    Ok(SpawnResult::new(api, tx))
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
        .no_storage_caching();

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


pub fn benchmark_shutdowns(c: &mut Criterion) {
    let mut group = c.benchmark_group("shutdowns");
    let rt = Runtime::new().unwrap();

    group.throughput(Throughput::Elements(1));
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for method in ["ipc", "http_local", "reth"].iter() {
        match *method {
            "ipc" => {
                let anvil_result = rt.block_on(async { spawn_ipc().await.unwrap() });
                group.bench_with_input(BenchmarkId::new("Shutdown", "ipc"), &rt, |b, rt| {
                    b.to_async(rt).iter(|| async {
                        system_shutdown(&anvil_result.api, anvil_result.transaction.clone()).await
                    })
                });
            },
            "http_local" => {
                let anvil_result = rt.block_on(async { spawn_http_local().await.unwrap() });
                group.bench_with_input(BenchmarkId::new("Shutdown", "http_local"), &rt, |b, rt| {
                    b.to_async(rt).iter(|| async {
                        system_shutdown(&anvil_result.api, anvil_result.transaction.clone()).await
                    })
                });
            },
            "reth" => {
                let anvil_result = rt.block_on(async { spawn_ethers_reth().await.unwrap() });
                group.bench_with_input(BenchmarkId::new("Shutdown", "reth"), &rt, |b, rt| {
                    b.to_async(rt).iter(|| async {
                        system_shutdown(&anvil_result.api, anvil_result.transaction.clone()).await
                    })
                });
            },
            _ => panic!("Unknown method"),
        }
    }
    group.finish();
}

pub fn benchmark_startups(c: &mut Criterion) {
    let mut group = c.benchmark_group("startups");
    let rt = Runtime::new().unwrap();

    group.throughput(Throughput::Elements(1));
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for method in ["ipc", "http_local", "reth"].iter() {
        match *method {
            "ipc" => {
                group.bench_with_input(BenchmarkId::new("Startup", "ipc"), &rt, |b, rt| {
                    b.to_async(rt).iter(|| async {
                        spawn_ipc().await.unwrap();
                    })
                });
            },
            "http_local" => {
                group.bench_with_input(BenchmarkId::new("Startup", "http_local"), &rt, |b, rt| {
                    b.to_async(rt).iter(|| async {
                        spawn_http_local().await.unwrap();
                    })
                });
            },
            "reth" => {
                group.bench_with_input(BenchmarkId::new("Startup", "reth"), &rt, |b, rt| {
                    b.to_async(rt).iter(|| async {
                        spawn_ethers_reth().await.unwrap();
                    })
                });
            },
            _ => panic!("Unknown method"),
        }
    }
    group.finish();
}

pub fn benchmark_startup_and_shutdown(c: &mut Criterion) {
    let mut group = c.benchmark_group("startup_and_shutdown");
    let rt = Runtime::new().unwrap();

    group.throughput(Throughput::Elements(1));
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for method in ["ipc", "http_local", "reth"].iter() {
        match *method {
            "ipc" => {
                group.bench_with_input(BenchmarkId::new("Startup_and_Shutdown", "ipc"), &rt, |b, rt| {
                    b.to_async(rt).iter(|| async {
                        let anvil_result = spawn_ipc().await.unwrap();
                        system_shutdown(&anvil_result.api, anvil_result.transaction.clone()).await;
                    })
                });
            },
            "http_local" => {
                group.bench_with_input(BenchmarkId::new("Startup_and_Shutdown", "http_local"), &rt, |b, rt| {
                    b.to_async(rt).iter(|| async {
                        let anvil_result = spawn_http_local().await.unwrap();
                        system_shutdown(&anvil_result.api, anvil_result.transaction.clone()).await;
                    })
                });
            },
            "reth" => {
                group.bench_with_input(BenchmarkId::new("Startup_and_Shutdown", "reth"), &rt, |b, rt| {
                    b.to_async(rt).iter(|| async {
                        let anvil_result = spawn_ethers_reth().await.unwrap();
                        system_shutdown(&anvil_result.api, anvil_result.transaction.clone()).await;
                    })
                });
            },
            _ => panic!("Unknown method"),
        }
    }
    group.finish();
}

criterion_group!(benches, benchmark_startups, benchmark_shutdowns, benchmark_startup_and_shutdown);
criterion_main!(benches);