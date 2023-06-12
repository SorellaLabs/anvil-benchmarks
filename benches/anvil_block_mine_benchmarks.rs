use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use anvil::eth::EthApi;
use std::pin::Pin;
use tokio::{macros::support::Future, runtime::Runtime};
mod utils;
use crate::utils::{
    block_simulation::{Block, *},
    SpawnResult,
};
use std::error::Error;

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

    let spawn_funcs: [(
        fn(u64) -> Pin<Box<dyn Future<Output = Result<SpawnResult, Box<dyn Error>>>>>,
        &str,
    ); 3] = [
        (|block_number| Box::pin(spawn_http_local(block_number)), "Local Http"),
        (|block_number| Box::pin(spawn_ipc(block_number)), "Ipc"),
        (|block_number| Box::pin(spawn_ethers_reth(block_number)), "ethers-reth middleware"),
    ];

    

    const START_BLOCK: u64 = 14556786;
    const END_BLOCK: u64 = 14556795;

    // Create a new runtime for the async task
    let rt = tokio::runtime::Runtime::new().unwrap();

    let provider = rt.block_on(spawn_ipc_provider());
    let blocks = rt.block_on(get_blocks(provider, START_BLOCK, END_BLOCK));

    for (spawn_func, description) in &spawn_funcs {
        group.sample_size(10).bench_function(format!("All blocks - {}", description), |b| {
            b.iter(|| {
                let rt = Runtime::new().unwrap();
                let spawn_func = spawn_func.clone();

                rt.block_on(async {
                    let spawn_result = spawn_func(START_BLOCK - 1).await.unwrap();
                    blocks_simulation(&blocks, &spawn_result.api).await;
                })
            });
        });

        for (i, block) in blocks.iter().enumerate() {
            let spawn_func = spawn_func.clone();

            group.sample_size(10).bench_with_input(
                BenchmarkId::new(format!("Block {} - {}", i, description), i),
                block,
                |b, block| {
                    b.iter(|| {
                        let rt = Runtime::new().unwrap();
                        let spawn_func = spawn_func.clone();

                        rt.block_on(async {
                            let spawn_result = spawn_func(block.block_number - 1).await.unwrap();
                            block_simulation(block, &spawn_result.api).await;
                        })
                    });
                },
            );
        }
    }


    /*
    let spawn_http_remote = |block_number| Box::pin(spawn_http_remote(block_number));
    
    group.sample_size(10).bench_function("All blocks - HTTP Remote", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();

            rt.block_on(async {
                let spawn_result = spawn_http_remote(START_BLOCK - 1).await.unwrap();
                blocks_simulation(&blocks, &spawn_result.api).await;
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
                        let spawn_result = spawn_http_remote(block.block_number - 1).await.unwrap();
                        block_simulation(block, &spawn_result.api).await;
                    })
                });
            },
        );
    }*/

    group.finish();
}
criterion_group!(benches, benchmarks);
criterion_main!(benches);
