pub mod mongodb;
pub mod postgresql;

mod underlying_db;

use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
use solana_transaction_status::EncodedConfirmedTransaction;
use underlying_db::UnderlyingDB;

use crate::StorageConfig;

use self::postgresql::PostgreSQL;
use super::config::StorageType;

pub struct Storage {
    underlying_db: Box<dyn UnderlyingDB + Send + Sync>,
}

impl Storage {
    pub fn new(storage_config: &StorageConfig) -> Storage {
        match storage_config.storage_type {
            StorageType::PostgreSQL => Storage {
                underlying_db: Box::new(PostgreSQL::new(&storage_config.database_url)),
            },
            StorageType::MongoDB => todo!(),
            StorageType::Undefined => todo!(),
        }
    }

    pub fn store_signatures_in_queue(
        &self,
        sgns: &[RpcConfirmedTransactionStatusWithSignature],
    ) -> Result<(), &str> {
        self.underlying_db.store_signatures_in_queue(sgns)
    }

    pub fn get_signature_from_queue(&self) -> Result<(i32, Option<String>), &str> {
        self.underlying_db.get_signature_from_queue()
    }

    pub fn mark_signature_as_loaded(&self, record_id: i32) {
        self.underlying_db.mark_signature_as_loaded(record_id)
    }

    pub fn store_transaction(
        &self,
        sign: &str,
        transn: EncodedConfirmedTransaction,
    ) -> Result<(), &str> {
        self.underlying_db.store_transaction(sign, transn)
    }
}
