
pub struct TransactionAnalyzer;
use solana_transaction_status::UiTransactionStatusMeta;

impl TransactionAnalyzer {
    pub fn analyze_transaction(
        transaction: &UiTransactionStatusMeta,
        wallet: &Pubkey,
    ) -> Result<TransactionAnalysis, BotError> {
        // TODO: implement properly.
        let is_people `
    }
}