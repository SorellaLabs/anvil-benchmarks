mod bindings;

use criterion::{criterion_group, criterion_main, Criterion};
use std::error::Error;

use criterion::async_executor::FuturesExecutor;

struct SpawnResult {
    provider: Arc<Provider<Ipc>>,
    api: EthApi,
    handle: NodeHandle,
}

impl SpawnResult {
    fn new(provider: Arc<Provider<Ipc>>, api: EthApi, handle: NodeHandle) -> Self {
        Self { provider, api, handle }
    }
}

pub fn benchmark_http_local(c: &mut Criterion) {
    c.bench_function("http_local_fork", |b| {
        b.to_async(FuturesExecutor).iter(|| {
            let spawn = spawn_http_local();

            system_shutdown(spawn.provider, spawn.api);
        })
    });
}

pub fn benchmark_http(c: &mut Criterion) {
    c.bench_function("http_fork", |b| {
        b.to_async(FuturesExecutor).iter(|| {
            let spawn = spawn_http_external();

            system_shutdown(spawn.provider, spawn.api);
        })
    });
}

pub fn benchmark_ipc(c: &mut Criterion) {
    c.bench_function("ipc_fork", |b| {
        b.to_async(FuturesExecutor).iter(|| {
            let spawn = spawn_ipc();

            system_shutdown(spawn.provider, spawn.api);
        })
    });
}

pub fn benchmark_reth(c: &mut Criterion) {
    c.bench_function("reth_fork", |b| {
        b.to_async(FuturesExecutor).iter(|| {
            let spawn = spawn_ethers_reth();

            system_shutdown(spawn.provider, spawn.api);
        })
    });
}

async fn system_shutdown(provider: Arc<Provider<Ipc>>, api: &EthApi) {
    let convex_sys = H160::from_str("0xF403C135812408BFbE8713b5A23a04b3D48AAE31").unwrap();
    let owner = H160::from_str("0x3cE6408F923326f81A7D7929952947748180f1E6").unwrap();

    api.anvil_set_balance(owner, U256::from(1e19 as u64)).await.unwrap();

    let tx = TransactionRequest {
        from: Some(owner),
        to: Some(convex_sys.into()),
        value: None,
        gas_price: Some(provider.get_gas_price().await.unwrap()),
        nonce: Some(provider.get_transaction_count(owner, None).await.unwrap()),
        gas: Some(28_000_000u64.into()),
        data: Some(ShutdownSystemCall {}.encode().into()),
        chain_id: Some(1.into()),
    };

    api.call(tx.into(), Some(BlockId::Number(14445961.into())), None).await.unwrap();
}

async fn spawn_with_config(config: NodeConfig) -> Result<SpawnResult, Box<dyn Error>> {
    let (api, handle) = spawn(config).await;

    api.anvil_auto_impersonate_account(true).await?;

    let provider = Arc::new(Provider::<Ipc>::connect_ipc(handle.ipc_path().unwrap()).await?);

    Ok(SpawnResult::new(provider, api, handle))
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
        .with_ipc(Some(None))
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

    let mut config = NodeConfig::default()
        .with_eth_rpc_url(Some(rpc_url.to_string()))
        .with_port(1299)
        .with_fork_block_number::<u64>(Some(14445961))
        .with_ipc(Some(None))
        .with_gas_limit(Some(GAS))
        .no_storage_caching();

    if TRACE_COUNT.load(Ordering::SeqCst) == 0 {
        config = config.with_tracing(true).with_steps_tracing(true);
        TRACE_COUNT.fetch_add(1, Ordering::SeqCst);
        TRACE_COUNT.load(Ordering::SeqCst);
    } else {
        config = config.silent().with_steps_tracing(false);
    }

    spawn_with_config(config).await
}

criterion_group!(benches, benchmark_http_local, benchmark_http, benchmark_ipc, benchmark_reth);
criterion_main!(benches);
