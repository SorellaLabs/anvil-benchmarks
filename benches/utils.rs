use anvil::{eth::EthApi, spawn, NodeConfig};
use ethers::prelude::*;
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
    use super::*;

    pub struct Block {
        pub txs: Vec<TransactionRequest>,
    }

    impl Block {
        fn new(txs: Vec<TransactionRequest>) -> Self {
            Self { txs }
        }
    }

    pub fn get_blocks() -> Vec<Block> {
        return Vec::new()
    }

    pub async fn spawn_with_config(config: NodeConfig) -> Result<SpawnResult, Box<dyn Error>> {
        let (api, handle) = spawn(config).await;
        let provider = Arc::new(Provider::<Ipc>::connect_ipc(handle.ipc_path().unwrap()).await?);
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
            .silent()
            .with_no_mining(true);

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
            .silent()
            .with_no_mining(true);

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
            .fork_request_timeout(Some(Duration::from_millis(85000)))
            .with_no_mining(true);

        spawn_with_config(config).await
    }
}

pub mod system_shutdown {
    use super::*;

    pub async fn spawn_with_config(config: NodeConfig) -> Result<SpawnResult, Box<dyn Error>> {
        let (api, handle) = spawn(config).await;
        let provider = Arc::new(Provider::<Ipc>::connect_ipc(handle.ipc_path().unwrap()).await?);
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
            .fork_request_timeout(Some(Duration::from_millis(85000)));

        spawn_with_config(config).await
    }
}
