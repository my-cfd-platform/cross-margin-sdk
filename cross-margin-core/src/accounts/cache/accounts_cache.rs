use crate::{accounts::CrossMarginAccount, CrossMarginError};

use super::AccountsStore;

pub struct AccountsCache<T>
where
    T: CrossMarginAccount + Clone,
{
    pub accounts_store: AccountsStore<T>,
}

impl<T> AccountsCache<T>
where
    T: CrossMarginAccount + Clone,
{
    pub fn new(accounts: Vec<T>) -> Self {
        AccountsCache {
            accounts_store: AccountsStore::new(accounts),
        }
    }

    pub fn get_account(&self, accounts_id: &str) -> Option<&T> {
        let account = self.accounts_store.get_account(accounts_id)?;

        return Some(account);
    }

    pub fn add_account(&mut self, account: T) -> T {
        service_sdk::metrics::gauge!("accounts_in_cache").increment(1);
        return self.accounts_store.add_account(account);
    }

    pub async fn get_accounts(&self, account: &[&str]) -> Vec<T> {
        let mut result = vec![];

        for account_id in account {
            if let Some(account) = self.accounts_store.get_account(account_id) {
                result.push(account.clone());
            }
        }

        return result;
    }

    pub async fn get_trader_accounts(&self, trader_id: &str) -> Option<Vec<T>> {
        let accounts = self.accounts_store.get_trader_accounts(trader_id)?;

        return Some(accounts.into_iter().map(|x| x.clone()).collect());
    }

    pub async fn get_all(&self) -> Vec<T> {
        return self
            .accounts_store
            .get_all()
            .into_iter()
            .map(|x| x.clone())
            .collect();
    }

    pub async fn update_balance(
        &mut self,
        account_id: &str,
        delta: f64,
        process_id: &str,
        allow_negative_balance: bool,
    ) -> Result<T, CrossMarginError> {
        let result = self
            .accounts_store
            .update_account(account_id, process_id, |account| {
                if account.get_balance() + delta < 0.0 && allow_negative_balance {
                    return Some(Err(CrossMarginError::NotEnoughBalance));
                }

                account.update_balance(delta);

                return Some(Ok(account.clone()));
            })
            .await?
            .ok_or(CrossMarginError::AccountNotFound)??;

        return Ok(result);
    }

    pub async fn update_trading_disabled(
        &mut self,
        account_id: &str,
        trading_disabled: bool,
        process_id: &str,
    ) -> Result<T, CrossMarginError> {
        let result = self
            .accounts_store
            .update_account(account_id, process_id, |account| {
                account.set_trading_disabled(trading_disabled);

                return Some(account.clone());
            })
            .await?
            .ok_or(CrossMarginError::AccountNotFound)?;

        return Ok(result);
    }

    pub async fn update_trading_group(
        &mut self,
        account_id: &str,
        trading_group: &str,
        process_id: &str,
    ) -> Result<T, CrossMarginError> {
        let result = self
            .accounts_store
            .update_account(account_id, process_id, |account| {
                account.update_trading_group(trading_group.to_string());

                return Some(account.clone());
            })
            .await?
            .ok_or(CrossMarginError::AccountNotFound)?;

        return Ok(result);
    }

    pub async fn update_leverage(
        &mut self,
        account_id: &str,
        leverage: f64,
        process_id: &str,
    ) -> Result<T, CrossMarginError> {
        let result = self
            .accounts_store
            .update_account(account_id, process_id, |account| {
                account.update_leverage(leverage);

                return Some(account.clone());
            })
            .await?
            .ok_or(CrossMarginError::AccountNotFound)?;

        return Ok(result);
    }

    pub async fn update_accounts<F>(
        &mut self,
        ids: &[&str],
        process_id: &str,
        update_command: impl Fn(&mut T) -> Option<F>,
    ) -> Vec<F> {
        return self
            .accounts_store
            .update_accounts(ids, process_id, update_command)
            .await;
    }
}
