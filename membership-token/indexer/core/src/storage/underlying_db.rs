use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
use solana_transaction_status::EncodedConfirmedTransaction;

pub trait UnderlyingDB {
    fn store_signatures_in_queue(
        &self,
        sgns: &[RpcConfirmedTransactionStatusWithSignature],
    ) -> Result<(), &str>;

    fn get_signature_from_queue(&self) -> Result<(i32, Option<String>), &str>;

    fn mark_signature_as_loaded(&self, record_id: i32);

    fn store_transaction(
        &self,
        sign: &str,
        transn: EncodedConfirmedTransaction,
    ) -> Result<(), &str>;
}
