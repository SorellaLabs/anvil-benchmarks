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
    use std::{collections::HashMap, str::FromStr};

    use csv::ReaderBuilder;
    use ethers::utils::parse_ether;

    use super::*;

    pub struct Block {
        pub txs: Vec<TransactionRequest>,
    }

    impl Block {
        fn new(txs: Vec<TransactionRequest>) -> Self {
            Self { txs }
        }
    }

    pub fn get_blocks(start_block: u64, end_block: u64) -> Vec<Block> {
        let mut blocks = Vec::new();
        for i in start_block..end_block+1 {
            blocks.push(get_block(i))
        }
        blocks
    }

    pub fn into_tx_request(row: Vec<String>) -> TransactionRequest {
    
        let data = match row[7].as_str() {
            "" | "0" | "0x" => vec![],
            _ => hex::decode(&row[7][2..]).unwrap(),
        };
    
        TransactionRequest { 
            from: Some(H160::from_str(&row[1][2..]).unwrap()), 
            to: Some(NameOrAddress::Name((row[2].clone()))), 
            gas: row[3].parse::<U256>().ok(),
            gas_price: parse_gas_price(&row[4]),
            value: parse_ether(&row[6]).ok(),
            data: Some(Bytes::from(data)),
            nonce: row[5].parse::<U256>().ok(), 
            chain_id: Some(U64::from_dec_str("1").unwrap())
        }
    }
    
    
    pub fn get_block(block_number: u64) -> Block {

        let block_times: HashMap<u64, u64> = [
            (17200000, 1683357287), 
            (17200001, 1683357299), 
            (17200002, 1683357311), 
            (17200003, 1683357323), 
            (17200004, 1683357335), 
            (17200005, 1683357347), 
            (17200006, 1683357359), 
            (17200007, 1683357371), 
            (17200008, 1683357383), 
            (17200009, 1683357395), 
            (17200010, 1683357407)
        ].iter().cloned().collect();

        let mut reader = ReaderBuilder::new()
            .delimiter(b',')
            .quote(b'"')
            .double_quote(true)
            .escape(None)
            .terminator(csv::Terminator::Any(b'\n'))
            .flexible(true)
            .from_path(env::var("BLOCKS_CSV").expect("ETH_IPC_PATH not found in .env"))
            .unwrap();
    
        let mut txs: Vec<TransactionRequest> = Vec::new();
        for row in reader.records() {
            let res = row.unwrap().deserialize::<Vec<String>>(None).unwrap();
            let timestamp: f64 = res[8].parse().unwrap();
            if  timestamp >= *block_times.get(&(block_number-1)).unwrap() as f64 && timestamp > *block_times.get(&block_number).unwrap() as f64 {
                txs.push(into_tx_request(res));
            }
        }
        
        Block::new(txs)
    }
    
    fn parse_gas_price(price: &str) -> Option<U256> {
        if price == "" {
            None
        } else {
            Some(U256::from_dec_str(&price).unwrap())
        }
    }

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
            .fork_request_timeout(Some(Duration::from_millis(65000)));

        spawn_with_config(config).await
    }
}
//cargo criterion --bench anvil_sys_shutdown_benchmark --message-format=json > anvil_sys_shutdown_benchmark.json