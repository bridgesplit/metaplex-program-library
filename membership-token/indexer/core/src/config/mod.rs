pub mod config_ini;
pub mod config_toml;
mod lodable_config;

use std::str::FromStr;

use crate::SolanaRpcClientConfig;
use lodable_config::LoadableConfig;
use solana_sdk::pubkey::Pubkey;

// ToDo: move to Storage module
#[derive(Debug, Clone, Copy)]
pub enum StorageType {
    PostgreSQL,
    MongoDB,
    Undefined,
}

#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub storage_type: StorageType,
    pub database_url: String,
}

#[derive(Debug, Clone, Copy)]
pub struct WorkersPoolConfig {
    pub nunmber_of_transaction_loaders: u8,
}

#[derive(Debug, Clone)]
pub struct Config {
    storage_config: StorageConfig,
    solana_rpc_client_config: SolanaRpcClientConfig,
    workers_pool_config: WorkersPoolConfig,
}

impl Config {
    pub fn new<'a, LC: std::default::Default>() -> Self
    where
        LC: LoadableConfig<'a>,
    {
        let mut lodable_config = LC::default();
        lodable_config.load().expect("Unable to load configuration");

        let storage_type = match lodable_config.get("DataStorage", "type") {
            Some("postgresql") => StorageType::PostgreSQL,
            Some("mongodb") => StorageType::MongoDB,
            _ => StorageType::Undefined,
        };

        let database_url =
            String::from_str(lodable_config.get("DataStorage", "database_url").unwrap()).unwrap();

        let storage_config = StorageConfig {
            storage_type,
            database_url,
        };

        let url =
            String::from_str(lodable_config.get("Contract", "endpoint_url").unwrap()).unwrap();

        let program_address =
            Pubkey::from_str(lodable_config.get("Contract", "id").unwrap()).unwrap();

        let solana_rpc_client_config = SolanaRpcClientConfig {
            url,
            program_address,
        };

        let nunmber_of_transaction_loaders: u8 = lodable_config
            .get("WorkersPool", "nunmber_of_transaction_loaders")
            .unwrap()
            .parse()
            .unwrap();

        let workers_pool_config = WorkersPoolConfig {
            nunmber_of_transaction_loaders,
        };

        Config {
            storage_config,
            solana_rpc_client_config,
            workers_pool_config,
        }
    }

    pub fn get_storage_config(&self) -> &StorageConfig {
        &self.storage_config
    }

    pub fn get_solana_rpc_client_config(&self) -> &SolanaRpcClientConfig {
        &self.solana_rpc_client_config
    }

    pub fn get_workers_pool_config(&self) -> &WorkersPoolConfig {
        &self.workers_pool_config
    }
}
