use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use anvil::eth::EthApi;
use std::pin::Pin;
use tokio::{macros::support::Future, runtime::Runtime};

mod utils;
use crate::utils::{
    block_simulation::{Block, *},
    SpawnResult,
};

async fn block_simulation(block: &Block, api: &EthApi) {
    for tx in &block.txs {
        api.send_transaction(tx.clone().into()).await.unwrap();
    }
    api.mine_one().await;
}

async fn blocks_simulation(blocks: &[Block], api: &EthApi) {
    for block in blocks {
        block_simulation(block, api).await;
    }
}

pub fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Block Simulation");

    let spawn_funcs: [(fn() -> Pin<Box<dyn Future<Output = SpawnResult>>>, &str); 3] = [
        (|| Box::pin(async { spawn_http_local().await.unwrap() }), "Local Http"),
        (|| Box::pin(async { spawn_ipc().await.unwrap() }), "Ipc"),
        (|| Box::pin(async { spawn_ethers_reth().await.unwrap() }), "ethers-reth middleware"),
    ];

    let spawn_http_remote = || Box::pin(async { spawn_http_remote().await.unwrap() });

    let blocks = get_blocks();

    for (spawn_func, description) in &spawn_funcs {
        group.bench_function(format!("All blocks - {}", description), |b| {
            b.iter(|| {
                let rt = Runtime::new().unwrap();
                let spawn_func = spawn_func.clone();

                rt.block_on(async {
                    let anvil_result = spawn_func().await;
                    blocks_simulation(&blocks, &anvil_result.api).await;
                })
            });
        });

        for (i, block) in blocks.iter().enumerate() {
            let spawn_func = spawn_func.clone();

            group.bench_with_input(
                BenchmarkId::new(format!("Block {} - {}", i, description), i),
                block,
                |b, block| {
                    b.iter(|| {
                        let rt = Runtime::new().unwrap();
                        let spawn_func = spawn_func.clone();

                        rt.block_on(async {
                            let anvil_result = spawn_func().await;
                            block_simulation(block, &anvil_result.api).await;
                        })
                    });
                },
            );
        }
    }

    group.sample_size(10).bench_function("All blocks - HTTP Remote", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();

            rt.block_on(async {
                let anvil_result = spawn_http_remote().await;
                blocks_simulation(&blocks, &anvil_result.api).await;
            })
        })
    });

    for (i, block) in blocks.iter().enumerate() {
        group.sample_size(10).bench_with_input(
            BenchmarkId::new(format!("Block {} - HTTP Remote", i), i),
            block,
            |b, block| {
                b.iter(|| {
                    let rt = Runtime::new().unwrap();

                    rt.block_on(async {
                        let anvil_result = spawn_http_remote().await;
                        block_simulation(block, &anvil_result.api).await;
                    })
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
