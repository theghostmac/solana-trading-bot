use solana_transaction_status::UiTransactionStatusMeta;
use solana_sdk::pubkey::Pubkey;
use crate::utils::error::BotError;

#[derive(Debug)]
pub struct TransactionAnalysis {
    pub wallet_involved: Pubkey,
    pub transaction_type: TransactionType,
    pub amount: Option<f64>,
    pub token_address: Option<String>,
    pub timestamp: i64,
}

#[derive(Debug)]
pub enum TransactionType {
    TokenPurchase,
    TokenSale,
    Transfer,
    Unknown,
}

pub struct TransactionAnalyzer;

impl TransactionAnalyzer {
    pub fn new() -> Self {
        TransactionAnalyzer
    }

    pub fn analyze_transaction(
        &self,
        transaction: &UiTransactionStatusMeta,
        wallet: &Pubkey,
    ) -> Result<TransactionAnalysis, BotError> {
        // Get current timestamp
        let timestamp = chrono::Utc::now().timestamp();

        // Basic analysis structure
        let mut analysis = TransactionAnalysis {
            wallet_involved: *wallet,
            transaction_type: TransactionType::Unknown,
            amount: None,
            token_address: None,
            timestamp,
        };

        // Analyze pre token balances and post token balances
        if let Some(pre_token_balances) = &transaction.pre_token_balances {
            if let Some(post_token_balances) = &transaction.post_token_balances {
                for (pre, post) in pre_token_balances.iter().zip(post_token_balances.iter()) {
                    if pre.owner == wallet.to_string() {
                        let pre_amount = pre.ui_token_amount.ui_amount.unwrap_or(0.0);
                        let post_amount = post.ui_token_amount.ui_amount.unwrap_or(0.0);

                        analysis.amount = Some((post_amount - pre_amount).abs());
                        analysis.token_address = Some(pre.mint.clone());

                        if post_amount > pre_amount {
                            analysis.transaction_type = TransactionType::TokenPurchase;
                        } else if post_amount < pre_amount {
                            analysis.transaction_type = TransactionType::TokenSale;
                        } else {
                            analysis.transaction_type = TransactionType::Transfer;
                        }
                    }
                }
            }
        }

        Ok(analysis)
    }
}