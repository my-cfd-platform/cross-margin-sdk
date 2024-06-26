use std::collections::HashMap;

use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{accounts::CrossMarginAccount, CrossMarginError};

pub struct AccountsStore<T>
where
    T: CrossMarginAccount + Clone,
{
    pub trader_index: HashMap<String, Vec<String>>,
    pub id_account_index: HashMap<String, T>,
}

impl<T> AccountsStore<T>
where
    T: CrossMarginAccount + Clone,
{
    pub fn new(accounts: Vec<T>) -> Self {
        let accounts_len = accounts.len();

        let mut trader_index = HashMap::new();
        let mut id_account_index = HashMap::new();

        for account in accounts {
            trader_index
                .entry(account.get_trader_id().to_string())
                .or_insert(Vec::new())
                .push(account.get_id().to_string());

            id_account_index.insert(account.get_id().to_string(), account);
        }

        service_sdk::metrics::gauge!("accounts_in_cache").set(accounts_len as f64);

        Self {
            trader_index,
            id_account_index,
        }
    }

    pub fn get_account(&self, accounts_id: &str) -> Option<&T> {
        return self.id_account_index.get(accounts_id);
    }

    pub fn get_trader_accounts(&self, trader_id: &str) -> Option<Vec<&T>> {
        let trader_accounts = self.trader_index.get(trader_id)?;

        return Some(
            trader_accounts
                .iter()
                .filter_map(|x| self.id_account_index.get(x))
                .collect(),
        );
    }

    pub fn add_account(&mut self, account: T) -> T {
        let trader_accounts = self
            .trader_index
            .entry(account.get_trader_id().to_string())
            .or_insert(vec![]);
        trader_accounts.push(account.get_id().to_string());
        self.id_account_index
            .insert(account.get_id().to_string(), account.clone());

        return account;
    }

    pub async fn update_accounts<F>(
        &mut self,
        ids: &[&str],
        process_id: &str,
        update_command: impl Fn(&mut T) -> Option<F>,
    ) -> Vec<F> {
        let mut result = vec![];
        let update_date = DateTimeAsMicroseconds::now();

        for id in ids {
            let account = self.id_account_index.get_mut(id.to_owned());

            if let Some(account) = account {
                if let Some(value) = update_command(account) {
                    result.push(value);
                }
                account.track_update(process_id, update_date);
            }
        }

        return result;
    }

    pub async fn update_account<F>(
        &mut self,
        id: &str,
        process_id: &str,
        update_command: impl Fn(&mut T) -> Option<F>,
    ) -> Result<Option<F>, CrossMarginError> {
        let update_date = DateTimeAsMicroseconds::now();

        let account = self.id_account_index.get_mut(id);

        if let Some(account) = account {
            let result = update_command(account);
            account.track_update(process_id, update_date);

            return Ok(result);
        }

        return Err(CrossMarginError::AccountNotFound);
    }

    pub fn get_all(&self) -> Vec<&T> {
        self.id_account_index.values().collect()
    }
}
