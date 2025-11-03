#![allow(clippy::too_many_arguments)]
use crate::{
    apis::{self, configuration::Configuration},
    config::LighterConfig,
    models::{
        DepositHistory, EnrichedTx, NextNonce, RespSendTx, RespSendTxBatch, TransferHistory, Txs,
        WithdrawHistory,
    },
    Result,
};

#[derive(Debug, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum AccountTxsBy {
    AccountIndex,
}

#[derive(Debug, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum BlockTxsBy {
    BlockCommitment,
    BlockHeight,
}

#[derive(Debug, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum DepositHistoryFilter {
    All,
    Pending,
    Claimable,
}

#[derive(Debug, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum TxBy {
    Hash,
    SequenceIndex,
}

#[derive(Debug, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum WithdrawHistoryFilter {
    All,
    Pending,
    Claimable,
}

#[derive(Debug)]
pub struct TransactionApi {
    config: apis::configuration::Configuration,
}

impl TransactionApi {
    pub fn new(config: &LighterConfig) -> Result<Self> {
        Ok(Self {
            config: Configuration::try_from(config)?,
        })
    }

    /// Get transactions of a specific account
    pub async fn account_txs(
        &self,
        limit: i64,
        by: AccountTxsBy,
        value: &str,
        authorization: Option<&str>,
        index: Option<i64>,
        types: Option<Vec<i32>>,
        auth: Option<&str>,
    ) -> Result<Txs> {
        let resp = apis::transaction_api::account_txs(
            &self.config,
            limit,
            &by.to_string(),
            value,
            authorization,
            index,
            types,
            auth,
        )
        .await
        .inspect_err(|e| tracing::error!("unable to call `account_txs`: {e}"))?;

        Ok(resp)
    }

    /// Get transactions in a block
    pub async fn block_txs(&self, by: BlockTxsBy, value: &str) -> Result<Txs> {
        let resp = apis::transaction_api::block_txs(&self.config, &by.to_string(), value)
            .await
            .inspect_err(|e| tracing::error!("unable to call `block_txs`: {e}"))?;

        Ok(resp)
    }

    /// Get deposit history
    pub async fn deposit_history(
        &self,
        account_index: i64,
        l1_address: &str,
        authorization: Option<&str>,
        auth: Option<&str>,
        cursor: Option<&str>,
        filter: Option<DepositHistoryFilter>,
    ) -> Result<DepositHistory> {
        let resp = apis::transaction_api::deposit_history(
            &self.config,
            account_index,
            l1_address,
            authorization,
            auth,
            cursor,
            filter.map(|v| v.to_string()).as_deref(),
        )
        .await
        .inspect_err(|e| tracing::error!("unable to call `deposit_history`: {e}"))?;

        Ok(resp)
    }

    /// Get next nonce for a specific account and api key
    pub async fn next_nonce(&self, account_index: i64, api_key_index: i32) -> Result<NextNonce> {
        let resp = apis::transaction_api::next_nonce(&self.config, account_index, api_key_index)
            .await
            .inspect_err(|e| tracing::error!("unable to call `next_nonce`: {e}"))?;

        Ok(resp)
    }

    /// You need to sign the transaction body before sending it to the server. More details can be found in the Get Started docs: [Get Started For Programmers](https://apidocs.lighter.xyz/docs/get-started-for-programmers)
    pub async fn send_tx(
        &self,
        tx_type: i32,
        tx_info: &str,
        price_protection: Option<bool>,
    ) -> Result<RespSendTx> {
        let resp = apis::transaction_api::send_tx(&self.config, tx_type, tx_info, price_protection)
            .await
            .inspect_err(|e| tracing::error!("unable to call `send_tx`: {e}"))?;

        Ok(resp)
    }

    /// You need to sign the transaction body before sending it to the server. More details can be found in the Get Started docs: [Get Started For Programmers](https://apidocs.lighter.xyz/docs/get-started-for-programmers)
    pub async fn send_tx_batch(&self, tx_types: &str, tx_infos: &str) -> Result<RespSendTxBatch> {
        let resp = apis::transaction_api::send_tx_batch(&self.config, tx_types, tx_infos)
            .await
            .inspect_err(|e| tracing::error!("uanble to call `send_tx_batch`: {e}"))?;

        Ok(resp)
    }

    /// Get transfer history
    pub async fn transfer_history(
        &self,
        account_index: i64,
        authorization: Option<&str>,
        auth: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<TransferHistory> {
        let resp = apis::transaction_api::transfer_history(
            &self.config,
            account_index,
            authorization,
            auth,
            cursor,
        )
        .await
        .inspect_err(|e| tracing::error!("unable to call `transfer_history`: {e}"))?;

        Ok(resp)
    }

    /// Get transaction by hash or sequence index
    pub async fn tx(&self, by: TxBy, value: &str) -> Result<EnrichedTx> {
        let resp = apis::transaction_api::tx(&self.config, &by.to_string(), value)
            .await
            .inspect_err(|e| tracing::error!("unable to call `tx`: {e}"))?;

        Ok(resp)
    }

    /// Get L1 transaction by L1 transaction hash
    pub async fn tx_from_l1_tx_hash(&self, hash: &str) -> Result<EnrichedTx> {
        let resp = apis::transaction_api::tx_from_l1_tx_hash(&self.config, hash)
            .await
            .inspect_err(|e| tracing::error!("unable to call `tx_by_l1_tx_hash`: {e}"))?;

        Ok(resp)
    }

    /// Get transactions which are already packed into blocks
    pub async fn txs(&self, limit: i64, index: Option<i64>) -> Result<Txs> {
        let resp = apis::transaction_api::txs(&self.config, limit, index)
            .await
            .inspect_err(|e| tracing::error!("unable to call `txs`: {e}"))?;

        Ok(resp)
    }

    /// Get withdraw history
    pub async fn withdraw_history(
        &self,
        account_index: i64,
        authorization: Option<&str>,
        auth: Option<&str>,
        cursor: Option<&str>,
        filter: Option<WithdrawHistoryFilter>,
    ) -> Result<WithdrawHistory> {
        let resp = apis::transaction_api::withdraw_history(
            &self.config,
            account_index,
            authorization,
            auth,
            cursor,
            filter.map(|v| v.to_string()).as_deref(),
        )
        .await
        .inspect_err(|e| tracing::error!("unable to call `withdraw_history`: {e}"))?;

        Ok(resp)
    }
}
