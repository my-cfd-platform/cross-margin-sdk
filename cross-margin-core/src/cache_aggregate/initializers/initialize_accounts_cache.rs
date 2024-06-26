use crate::{AccountsCache, CrossMarginAccount};

pub async fn initialize_account_cache<T: CrossMarginAccount + Clone>(
    accounts: Vec<T>,
) -> AccountsCache<T> {
    return AccountsCache::new(accounts);
}
