use anvil::{eth::EthApi, spawn, NodeConfig};
use anvil_benchmarks::bindings::convex::{convex_mod::*, ShutdownSystemCall};
use ethers::{
    abi::AbiEncode,
    prelude::{gas_oracle::blocknative::GasEstimate, *},
};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc, time::Instant};
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() {
    let (provider, api) = spawn_rpc().await;
    system_shutdown(provider, &api).await;

    let fork = api.get_fork().unwrap();
    fork.reset(
        Some(env::var("ETH_RPC_URL").expect("ETH_RPC_URL not found in .env")),
        BlockId::Number(14445961.into()),
    )
    .await
    .unwrap();
    fork.storage_write().clear();
    println!("Fork cache cleared");    
}

async fn spawn_rpc() -> (Arc<Provider<Ipc>>, EthApi) {
    let config = NodeConfig::default()
        .with_eth_rpc_url(Some(env::var("ETH_RPC_URL").expect("ETH_RPC_URL not found in .env")))
        .with_fork_block_number::<u64>(Some(14445961))
        .with_ipc(Some(None))
        .with_steps_tracing(true)
        .with_gas_limit(Some(30000000000 as u64));

    // Spawn the node with the custom config
    let (api, handle) = spawn(config).await;

    api.anvil_auto_impersonate_account(true).await.unwrap();
    let provider =
        Arc::new(Provider::<Ipc>::connect_ipc(handle.ipc_path().unwrap()).await.unwrap());
    (provider, api)
}

async fn system_shutdown(provider: Arc<Provider<Ipc>>, api: &EthApi) {
    let convex_sys: H160 = "0xF403C135812408BFbE8713b5A23a04b3D48AAE31".parse().unwrap();
    let owner: H160 = "0x3cE6408F923326f81A7D7929952947748180f1E6".parse().unwrap();

    api.anvil_set_balance(owner, U256::from(1e19 as u64)).await.unwrap();

    let shutdown = ShutdownSystemCall {}.encode().into();

    let nonce = provider.get_transaction_count(owner, None).await.unwrap();
    let gas_price = provider.get_gas_price().await.unwrap();

    let tx = TransactionRequest {
        from: Some(owner.into()),
        to: Some(convex_sys.into()),
        value: None,
        gas_price: Some(gas_price),
        nonce: Some(nonce),
        gas: Some(28_000_000u64.into()),
        data: Some(shutdown),
        chain_id: Some(1.into()),
    };

    let start = Instant::now();
    api.send_transaction(tx.clone().into()).await.unwrap();
    let duration = start.elapsed();
    println!("Transaction took: {:?}", duration);

    let start = Instant::now();
    api.call(
        tx.into(),
        Some(BlockId::Number(14445961.into())),
        None,
    ).await.unwrap();

    let duration = start.elapsed();

    println!("Call took: {:?}", duration);


}
