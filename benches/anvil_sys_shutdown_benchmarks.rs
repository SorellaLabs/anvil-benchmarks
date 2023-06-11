use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

mod bindings;
use bindings::convex::ShutdownSystemCall;

mod utils;
use crate::utils::{system_shutdown::*, SpawnResult};
use anvil::eth::EthApi;
use ethers::{abi::AbiEncode, prelude::*};
use std::{pin::Pin, sync::Arc};
use tokio::{macros::support::Future, runtime::Runtime};


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

pub fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Convex system shutdown simulation");

    // An array of async functions that spawn nodes with different configurations
    let spawn_funcs: [(fn() -> Pin<Box<dyn Future<Output = SpawnResult>>>, &str); 3] = [
        (|| Box::pin(async { spawn_http_local().await.unwrap() }), "Local Http"),
        (|| Box::pin(async { spawn_ipc().await.unwrap() }), "Ipc"),
        (|| Box::pin(async { spawn_ethers_reth().await.unwrap() }), "ethers-reth"),
    ];

    for (spawn_func, description) in spawn_funcs.iter() {
        let spawn_func = spawn_func.clone();

        group.sample_size(1000).bench_function(
            BenchmarkId::new("System shutdown", description),
            move |b| {
                b.iter(|| {
                    let rt = Runtime::new().unwrap();
                    let spawn_func = spawn_func.clone();

                    rt.block_on(async {
                        // Spawn a new node with the appropriate configuration
                        let anvil_result = spawn_func().await;
                        system_shutdown(&anvil_result.api, anvil_result.provider.clone()).await
                    })
                })
            },
        );
    }

    /*// Special case for HTTP Remote due to rate limiting.
    let spawn_http_remote = || Box::pin(async { spawn_http_remote().await.unwrap() });
    group.sample_size(10).bench_function(
        BenchmarkId::new("System shutdown", "HTTP Remote"),
        move |b| {
            b.iter(|| {
                let rt = Runtime::new().unwrap();
                let spawn_func = spawn_http_remote.clone();

                rt.block_on(async {
                    // Spawn a new node with the appropriate configuration
                    let anvil_result = spawn_func().await;
                    system_shutdown(&anvil_result.api, anvil_result.provider.clone()).await
                })
            })
        },
    );*/

    group.finish();
}
criterion_group!(benches, benchmarks);
criterion_main!(benches);
