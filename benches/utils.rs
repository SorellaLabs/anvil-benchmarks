use anvil::{eth::EthApi, spawn, NodeConfig};
use ethers::{
    prelude::*,
    providers::{Ipc, Provider},
    types::{NameOrAddress, Transaction, TransactionRequest, U64},
};
use std::{env, error::Error, sync::Arc, time::Duration};

const GAS: u64 = 30_000_000;

pub struct SpawnResult {
    pub api: EthApi,
    pub provider: Arc<Provider<Ipc>>,
}

impl SpawnResult {
    pub fn new(api: EthApi, provider: Arc<Provider<Ipc>>) -> Self {
        Self { api, provider }
    }
}

pub mod block_simulation {

    pub async fn spawn_ipc_provider() -> Provider<Ipc> {
        let ipc_path = std::env::var("ETH_IPC_PATH").expect("ETH_IPC_PATH not found in .env");
        Provider::connect_ipc(ipc_path).await.unwrap()
    }
    use super::*;

    pub struct Block {
        pub block_number: u64,
        pub txs: Vec<TransactionRequest>,
    }

    impl Block {
        fn new(block_number: u64, txs: Vec<TransactionRequest>) -> Self {
            Self { block_number, txs }
        }
    }

    fn into_tx_request(tx: Transaction) -> TransactionRequest {
        TransactionRequest {
            from: Some(tx.from),
            to: tx.to.map(NameOrAddress::Address),
            gas: Some(tx.gas),
            gas_price: tx.gas_price,
            value: Some(tx.value),
            data: Some(tx.input),
            nonce: Some(tx.nonce),
            chain_id: tx.chain_id.map(|n| U64::from(n.low_u64())),
            #[cfg(feature = "celo")]
            fee_currency: tx.fee_currency,
            #[cfg(feature = "celo")]
            gateway_fee_recipient: tx.gateway_fee_recipient,
            #[cfg(feature = "celo")]
            gateway_fee: tx.gateway_fee,
        }
    }

    async fn get_block(provider: &Provider<Ipc>, block_number: u64) -> Block {
        let block = provider.get_block_with_txs(block_number).await.unwrap().unwrap();
        let txs: Vec<TransactionRequest> =
            block.transactions.into_iter().map(into_tx_request).collect::<Vec<_>>();
        Block::new(block_number, txs)
    }

    pub async fn get_blocks(
        provider: Provider<Ipc>,
        start_block: u64,
        end_block: u64,
    ) -> Vec<Block> {
        let mut blocks = Vec::new();
        for i in start_block..end_block + 1 {
            let block = get_block(&provider, i).await;
            blocks.push(block);
        }
        blocks
    }

    pub async fn spawn_with_config(config: NodeConfig) -> Result<SpawnResult, Box<dyn Error>> {
        let (api, handle) = spawn(config).await;
        let provider = Arc::new(Provider::<Ipc>::connect_ipc(handle.ipc_path().unwrap()).await?);
        api.anvil_auto_impersonate_account(true).await.unwrap();
        Ok(SpawnResult::new(api, provider))
    }
    pub async fn spawn_http_local(block_number: u64) -> Result<SpawnResult, Box<dyn Error>> {
        spawn_http(true, block_number).await
    }

    pub async fn spawn_http_remote(block_number: u64) -> Result<SpawnResult, Box<dyn Error>> {
        spawn_http(false, block_number).await
    }

    pub async fn spawn_ipc(fork_block_number: u64) -> Result<SpawnResult, Box<dyn Error>> {
        let ipc_path = env::var("ETH_IPC_PATH").expect("ETH_IPC_PATH not found in .env");
        let config = NodeConfig::default()
            .with_eth_ipc_path(Some(ipc_path.to_string()))
            .with_fork_block_number::<u64>(Some(fork_block_number))
            .with_ipc(Some(None))
            .with_gas_limit(Some(GAS))
            .no_storage_caching()
            .with_steps_tracing(false)
            .with_tracing(false)
            .silent()
            .with_no_mining(true);

        spawn_with_config(config).await
    }

    pub async fn spawn_ethers_reth(fork_block_number: u64) -> Result<SpawnResult, Box<dyn Error>> {
        let ipc_path = env::var("ETH_IPC_PATH").expect("ETH_IPC_PATH not found in .env");
        let db_path = env::var("ETH_DB_PATH").expect("ETH_DB_PATH not found in .env");

        let config = NodeConfig::default()
            .with_eth_ipc_path(Some(ipc_path.to_string()))
            .with_eth_reth_db(Some(db_path.to_string()))
            .with_fork_block_number::<u64>(Some(fork_block_number))
            .with_ipc(Some(None))
            .with_gas_limit(Some(GAS))
            .no_storage_caching()
            .with_steps_tracing(false)
            .with_tracing(false)
            .silent()
            .with_no_mining(true);

        spawn_with_config(config).await
    }

    pub async fn spawn_http(
        local: bool,
        fork_block_number: u64,
    ) -> Result<SpawnResult, Box<dyn Error>> {
        let rpc_url = match local {
            true => env::var("ETH_RPC_URL_LOCAL").expect("ETH_RPC_URL_LOCAL not found in .env"),
            false => env::var("ETH_RPC_URL").expect("ETH_RPC_URL not found in .env"),
        };

        let config = NodeConfig::default()
            .with_eth_rpc_url(Some(rpc_url.to_string()))
            .with_port(1299)
            .with_fork_block_number::<u64>(Some(fork_block_number))
            .with_ipc(Some(None))
            .with_gas_limit(Some(GAS))
            .no_storage_caching()
            .with_steps_tracing(false)
            .with_tracing(false)
            .silent()
            .fork_compute_units_per_second(Some(2700))
            .fork_request_timeout(Some(Duration::from_millis(5000)))
            .with_no_mining(true);

        spawn_with_config(config).await
    }
}

pub mod system_shutdown {
    use super::*;

    pub async fn spawn_with_config(config: NodeConfig) -> Result<SpawnResult, Box<dyn Error>> {
        let (api, handle) = spawn(config).await;
        let provider = Arc::new(Provider::<Ipc>::connect_ipc(handle.ipc_path().unwrap()).await?);
        api.anvil_auto_impersonate_account(true).await.unwrap();
        Ok(SpawnResult::new(api, provider))
    }
    pub async fn spawn_http_local() -> Result<SpawnResult, Box<dyn Error>> {
        spawn_http(true).await
    }

    pub async fn spawn_http_remote() -> Result<SpawnResult, Box<dyn Error>> {
        spawn_http(false).await
    }

    pub async fn spawn_ipc() -> Result<SpawnResult, Box<dyn Error>> {
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

    pub async fn spawn_ethers_reth() -> Result<SpawnResult, Box<dyn Error>> {
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

    pub async fn spawn_http(local: bool) -> Result<SpawnResult, Box<dyn Error>> {
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
            .fork_compute_units_per_second(Some(2700))
            .fork_request_timeout(Some(Duration::from_millis(90000)));

        spawn_with_config(config).await
    }
}
//cargo criterion --bench anvil_sys_shutdown_benchmark --message-format=json >
// anvil_sys_shutdown_benchmark.json
