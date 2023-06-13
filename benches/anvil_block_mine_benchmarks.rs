use std::{error::Error, pin::Pin};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use tokio::{macros::support::Future, runtime::Runtime};
use num_format::{Locale, ToFormattedString};
// Local
use anvil::eth::EthApi;
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
    let spawn_funcs: [(
        fn(u64) -> Pin<Box<dyn Future<Output = Result<SpawnResult, Box<dyn Error>>>>>,
        &str,
    ); 3] = [
        (|block_number| Box::pin(spawn_http_local(block_number)), "Local_Http"),
        (|block_number| Box::pin(spawn_ipc(block_number)), "Ipc"),
        (|block_number| Box::pin(spawn_ethers_reth(block_number)), "ethers_reth_middleware"),
    ];

    const START_BLOCK: u64 = 14556786;
    const END_BLOCK: u64 = 14556795;

    let rt = tokio::runtime::Runtime::new().unwrap();
    let provider = rt.block_on(spawn_ipc_provider());
    let blocks = rt.block_on(get_blocks(provider, START_BLOCK, END_BLOCK));

    // Single Block Simulation Benchmarking
    {
        let mut group = c.benchmark_group("Individual Block Simulation");

        for (spawn_func, description) in &spawn_funcs {
            for (i, block) in blocks.iter().enumerate() {
                let spawn_func = spawn_func.clone();

                group.sample_size(10).bench_with_input(
                    BenchmarkId::new(
                        *description,
                        format!("Block: {}, TotalGas: {}", i.to_formatted_string(&Locale::en), block.gas_used.to_formatted_string(&Locale::en)),
                    ),
                    block,
                    |b, block| {
                        b.iter(|| {
                            let rt = Runtime::new().unwrap();
                            let spawn_func = spawn_func.clone();

                            rt.block_on(async {
                                let spawn_result =
                                    spawn_func(block.block_number - 1).await.unwrap();
                                block_simulation(block, &spawn_result.api).await;
                            })
                        });
                    },
                );
            }
        }

        group.finish();
    }

    // Sequential Block Simulation
    {
        let mut group = c.benchmark_group("Sequential Simulation");

        for (spawn_func, description) in &spawn_funcs {
            group.sample_size(10).bench_function(
                BenchmarkId::new(*description, "Blocks 14,556,786 -> 14,556,795"),
                |b| {
                    b.iter(|| {
                        let rt = Runtime::new().unwrap();
                        let spawn_func = spawn_func.clone();

                        rt.block_on(async {
                            let spawn_result = spawn_func(START_BLOCK - 1).await.unwrap();
                            blocks_simulation(&blocks, &spawn_result.api).await;
                        })
                    });
                },
            );
        }
        group.finish();
    }

    // HTTP Remote benchmark - Commented due to rate limiting issues.
    // To run this, ensure you have an expensive RPC subscription.
    /*
    {
        let mut group = c.benchmark_group("HTTP Remote Simulation");
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
                BenchmarkId::new(format!("Block_{} - HTTP Remote", i), block),
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
        }

        group.finish();
    }
    */
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
