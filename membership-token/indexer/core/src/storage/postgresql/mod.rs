pub mod models;
pub mod schema;

use self::models::{NewSignature, NewTransaction};
use self::schema::{signatures, signatures::dsl::*, transactions::dsl::*};
use diesel::{pg::PgConnection, prelude::*};

use std::sync::Mutex;

use super::underlying_db::UnderlyingDB;

pub struct PostgreSQL {
    connection: Mutex<PgConnection>,
}

impl PostgreSQL {
    pub fn new(database_url: &str) -> Self {
        let connection = establish_connection(database_url);
        PostgreSQL {
            connection: Mutex::new(connection),
        }
    }
}

impl UnderlyingDB for PostgreSQL {
    fn store_signatures_in_queue(
        &self,
        sgns: &[solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature],
    ) -> Result<(), &str> {
        for transaction_status in sgns.iter() {
            let new_signature = NewSignature {
                signature: &transaction_status.signature,
                slot: transaction_status.slot as i32,
                err: &format_or_empty(transaction_status.err.as_ref()),
                memo: &format_or_empty(transaction_status.memo.as_ref()),
                block_time: transaction_status.block_time.unwrap_or_default() as i32,
                confirmation_status: &format_or_empty(
                    transaction_status.confirmation_status.as_ref(),
                ),
                loading_status: 0_i32, // In queue
            };

            diesel::insert_into(signatures)
                .values(&new_signature)
                .execute(&*self.connection.lock().unwrap())
                .unwrap();
        }
        Ok(())
    }

    fn get_signature_from_queue(&self) -> Result<(i32, Option<String>), &str> {
        let result = signatures
            .filter(loading_status.eq(0))
            .select((signatures::columns::id, signatures::columns::signature))
            .first::<(i32, Option<String>)>(&*self.connection.lock().unwrap());

        if let Ok(..) = result {
            diesel::update(signatures.find(result.as_ref().unwrap().0))
                .set(loading_status.eq(1))
                .execute(&*self.connection.lock().unwrap())
                .unwrap();

            Ok(result.unwrap())
        } else {
            Err("")
        }
    }

    fn mark_signature_as_loaded(&self, record_id: i32) {
        diesel::update(signatures.find(record_id))
            .set(loading_status.eq(2))
            .execute(&*self.connection.lock().unwrap())
            .unwrap();
    }

    fn store_transaction(
        &self,
        sign: &str,
        transn: solana_transaction_status::EncodedConfirmedTransaction,
    ) -> Result<(), &str> {
        let new_transaction = NewTransaction {
            signature: sign,
            slot: transn.slot as i32,
            transaction: &format_or_empty(Some(transn.transaction)),
            block_time: transn.block_time.unwrap_or_default() as i32,
        };

        diesel::insert_into(transactions)
            .values(&new_transaction)
            .execute(&*self.connection.lock().unwrap())
            .unwrap();

        Ok(())
    }
}

fn establish_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connetcing to database_url)"))
}

fn format_or_empty<T: std::fmt::Debug>(val: Option<T>) -> String {
    if val.is_some() {
        format!("{:?}", val.as_ref().unwrap())
    } else {
        String::from("")
    }
}
