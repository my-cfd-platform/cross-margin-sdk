use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use super::{CrossMarginCacheIndexGenerator, CrossMarginPositionsCacheQueryBuilder, CrossMarginPositionsOneOfBulkQueryBuilder};

#[derive(Debug)]
pub struct CrossMarginPositionsCacheIndexes {
    pub base: HashMap<String, HashSet<Arc<String>>>,
    pub quote: HashMap<String, HashSet<Arc<String>>>,
    pub collateral: HashMap<String, HashSet<Arc<String>>>,
    pub client_identification: HashMap<String, HashSet<Arc<String>>>,
    pub account_identification: HashMap<String, HashSet<Arc<String>>>,
}

impl CrossMarginPositionsCacheIndexes {
    pub fn new() -> Self {
        Self {
            base: HashMap::new(),
            quote: HashMap::new(),
            collateral: HashMap::new(),
            client_identification: HashMap::new(),
            account_identification: HashMap::new(),
        }
    }

    pub fn add_index(&mut self, target: &impl CrossMarginCacheIndexGenerator) {
        let id = Arc::new(target.get_id_index());
        let base = target.get_base_index();
        let quote = target.get_quote_index();
        let collateral = target.get_collateral_index();
        let client = target.get_client_identification_index();
        let account = target.get_account_identification_index();

        Self::add_single_index(&mut self.base, id.clone(), base);
        Self::add_single_index(&mut self.quote, id.clone(), quote);
        Self::add_single_index(&mut self.collateral, id.clone(), collateral);
        Self::add_single_index(&mut self.client_identification, id.clone(), client);
        Self::add_single_index(&mut self.account_identification, id.clone(), account);
    }

    pub fn remove_index(&mut self, indx: &str) {
        let id = Arc::new(indx.to_string());
        Self::remove_index_single(&mut self.base, id.clone());
        Self::remove_index_single(&mut self.quote, id.clone());
        Self::remove_index_single(&mut self.collateral, id.clone());
        Self::remove_index_single(&mut self.client_identification, id.clone());
        Self::remove_index_single(&mut self.account_identification, id.clone());
    }

    fn remove_index_single(indexses: &mut HashMap<String, HashSet<Arc<String>>>, id: Arc<String>) {
        for (_, set) in indexses {
            set.remove(&id);
        }
    }

    pub fn query(&self, query: &CrossMarginPositionsCacheQueryBuilder) -> HashSet<Arc<String>> {
        let mut sets = vec![];

        if let Some(base) = &query.base {
            if let Some(base_ids) = self.base.get(base) {
                sets.push(base_ids.clone());
            }
        }

        if let Some(quote) = &query.quote {
            if let Some(quote_ids) = self.quote.get(quote) {
                sets.push(quote_ids.clone());
            }
        }

        if let Some(collateral) = &query.collateral {
            if let Some(collateral_ids) = self.collateral.get(collateral) {
                sets.push(collateral_ids.clone());
            }
        }

        if let Some(account) = &query.account {
            if let Some(account_ids) = self.account_identification.get(account) {
                sets.push(account_ids.clone());
            }
        }

        if let Some(client) = &query.client {
            if let Some(client_ids) = self.client_identification.get(client) {
                sets.push(client_ids.clone());
            }
        }

        let mut to_search = sets
            .into_iter()
            .filter_map(|x| {
                if x.len() > 0 {
                    return Some(x);
                };

                return None;
            })
            .collect::<Vec<_>>();

        let filters = query.filters_count();

        if to_search.len() == 0 {
            return HashSet::default();
        }

        if filters == 1 {
            return to_search[0].clone();
        }

        if to_search.len() == 1 && filters > 1 {
            return HashSet::default();
        }

        let mut result = to_search[0].clone();

        for set in to_search.iter_mut().skip(1) {
            result = result.intersection(set).cloned().collect();
        }
        return result;
    }

    pub fn bulk_query(&self, query: &CrossMarginPositionsOneOfBulkQueryBuilder) -> HashSet<Arc<String>> {
        let mut result = HashSet::new();

        if let Some(base) = &query.base {
            for base in base {
                if let Some(base_ids) = self.base.get(base) {
                    result.extend(base_ids.clone());
                }
            }
        }

        if let Some(quote) = &query.quote {
            for quote in quote {
                if let Some(quote_ids) = self.quote.get(quote) {
                    result.extend(quote_ids.clone());
                }
            }
        }

        if let Some(collateral) = &query.collateral {
            for collateral in collateral {
                if let Some(collateral_ids) = self.collateral.get(collateral) {
                    result.extend(collateral_ids.clone());
                }
            }
        }

        if let Some(account) = &query.account {
            for account in account {
                if let Some(account_ids) = self.account_identification.get(account) {
                    result.extend(account_ids.clone());
                }
            }
        }

        if let Some(client) = &query.client {
            for client in client {
                if let Some(client_ids) = self.client_identification.get(client) {
                    result.extend(client_ids.clone());
                }
            }
        }

        return result;
    }

    fn add_single_index(
        indexses: &mut HashMap<String, HashSet<Arc<String>>>,
        id: Arc<String>,
        value: Option<String>,
    ) {
        if let Some(value) = value {
            let set = indexses.entry(value).or_insert_with(HashSet::new);
            set.insert(id);
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::positions::{CrossMarginCacheIndexGenerator, CrossMarginPositionsCacheIndexes, CrossMarginPositionsCacheQueryBuilder};


    struct TestIndexStruct {
        pub id: String,
        pub base: String,
        pub quote: String,
        pub collateral: String,
        pub client_ident: String,
        pub account_ident: String,
    }

    impl TestIndexStruct {
        pub fn new(
            id: &str,
            base: &str,
            quote: &str,
            collateral: &str,
            client_ident: &str,
            account_ident: &str,
        ) -> Self {
            Self {
                id: id.to_string(),
                base: base.to_string(),
                quote: quote.to_string(),
                collateral: collateral.to_string(),
                client_ident: client_ident.to_string(),
                account_ident: account_ident.to_string(),
            }
        }
    }

    impl CrossMarginCacheIndexGenerator for TestIndexStruct {
        fn get_id_index(&self) -> String {
            self.id.clone()
        }

        fn get_base_index(&self) -> Option<String> {
            Some(self.base.clone())
        }

        fn get_quote_index(&self) -> Option<String> {
            Some(self.quote.clone())
        }

        fn get_collateral_index(&self) -> Option<String> {
            Some(self.collateral.clone())
        }

        fn get_client_identification_index(&self) -> Option<String> {
            Some(self.client_ident.clone())
        }

        fn get_account_identification_index(&self) -> Option<String> {
            Some(self.account_ident.clone())
        }
    }

    #[test]
    fn test_search_by_client_ident_single() {
        let mut cache = CrossMarginPositionsCacheIndexes::new();

        let query = CrossMarginPositionsCacheQueryBuilder::new().with_client("client_ident");

        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id2",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id3",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        let result = cache.query(&query);

        assert_eq!(result.len(), 3);

        assert!(result.contains(&"test_id1".to_string()));
        assert!(result.contains(&"test_id2".to_string()));
        assert!(result.contains(&"test_id3".to_string()));
    }

    #[test]
    fn test_search_by_client_ident_few() {
        let mut cache = CrossMarginPositionsCacheIndexes::new();
        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id2",
            "base",
            "quote",
            "collateral",
            "client_ident2",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id3",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id4",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id5",
            "base",
            "quote",
            "collateral",
            "client_ident2",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id6",
            "base",
            "quote",
            "collateral",
            "client_ident3",
            "account_ident",
        ));

        let query1 = CrossMarginPositionsCacheQueryBuilder::new().with_client("client_ident");

        let query2 = CrossMarginPositionsCacheQueryBuilder::new().with_client("client_ident2");

        let query3 = CrossMarginPositionsCacheQueryBuilder::new().with_client("client_ident3");

        let result1 = cache.query(&query1);
        let result2 = cache.query(&query2);
        let result3 = cache.query(&query3);

        assert_eq!(result1.len(), 3);
        assert_eq!(result2.len(), 2);
        assert_eq!(result3.len(), 1);

        assert!(result1.contains(&"test_id1".to_string()));
        assert!(result1.contains(&"test_id3".to_string()));
        assert!(result1.contains(&"test_id4".to_string()));

        assert!(result2.contains(&"test_id2".to_string()));
        assert!(result2.contains(&"test_id5".to_string()));

        assert!(result3.contains(&"test_id6".to_string()));
    }

    #[test]
    fn test_search_by_client_account_single() {
        let mut cache = CrossMarginPositionsCacheIndexes::new();

        let query = CrossMarginPositionsCacheQueryBuilder::new().with_account("account_ident");

        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id2",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id3",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        let result = cache.query(&query);

        assert_eq!(result.len(), 3);

        assert!(result.contains(&"test_id1".to_string()));
        assert!(result.contains(&"test_id2".to_string()));
        assert!(result.contains(&"test_id3".to_string()));
    }

    #[test]
    fn test_search_by_client_account_few() {
        let mut cache = CrossMarginPositionsCacheIndexes::new();
        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id2",
            "base",
            "quote",
            "collateral",
            "client_ident2",
            "account_ident2",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id3",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id4",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id5",
            "base",
            "quote",
            "collateral",
            "client_ident2",
            "account_ident2",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id6",
            "base",
            "quote",
            "collateral",
            "client_ident3",
            "account_ident3",
        ));

        let query1 = CrossMarginPositionsCacheQueryBuilder::new().with_account("account_ident");

        let query2 = CrossMarginPositionsCacheQueryBuilder::new().with_account("account_ident2");

        let query3 = CrossMarginPositionsCacheQueryBuilder::new().with_account("account_ident3");

        let result1 = cache.query(&query1);
        let result2 = cache.query(&query2);
        let result3 = cache.query(&query3);

        assert_eq!(result1.len(), 3);
        assert_eq!(result2.len(), 2);
        assert_eq!(result3.len(), 1);

        assert!(result1.contains(&"test_id1".to_string()));
        assert!(result1.contains(&"test_id3".to_string()));
        assert!(result1.contains(&"test_id4".to_string()));

        assert!(result2.contains(&"test_id2".to_string()));
        assert!(result2.contains(&"test_id5".to_string()));

        assert!(result3.contains(&"test_id6".to_string()));
    }

    #[test]
    fn test_search_by_account_and_base() {
        let mut cache = CrossMarginPositionsCacheIndexes::new();
        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id2",
            "base1",
            "quote",
            "collateral",
            "client_ident2",
            "account_ident1",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id3",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id4",
            "base3",
            "quote",
            "collateral",
            "client_ident",
            "account_ident3",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id5",
            "base3",
            "quote",
            "collateral",
            "client_ident2",
            "account_ident3",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id6",
            "base3",
            "quote",
            "collateral",
            "client_ident3",
            "account_ident3",
        ));

        let query1 = CrossMarginPositionsCacheQueryBuilder::new()
            .with_base("base3")
            .with_account("account_ident3");

        let query2 = CrossMarginPositionsCacheQueryBuilder::new()
            .with_base("base2")
            .with_account("account_ident2");

        let result1 = cache.query(&query1);
        let result2 = cache.query(&query2);

        assert_eq!(result1.len(), 3);
        assert_eq!(result2.len(), 0);

        assert!(result1.contains(&"test_id6".to_string()));
        assert!(result1.contains(&"test_id5".to_string()));
        assert!(result1.contains(&"test_id4".to_string()));
    }

    #[test]
    fn test_search_by_all() {
        let mut cache = CrossMarginPositionsCacheIndexes::new();
        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "base",
            "quote3",
            "collateral",
            "client_ident3",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id2",
            "base1",
            "quote",
            "collateral0",
            "client_ident2",
            "account_ident1",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id3",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        //1 - test_id1
        let query1 = CrossMarginPositionsCacheQueryBuilder::new()
            .with_base("base")
            .with_quote("quote3")
            .with_account("account_ident");

        //2 - test_id1, test_id3
        let query2 = CrossMarginPositionsCacheQueryBuilder::new()
            .with_collateral("collateral")
            .with_base("base")
            .with_account("account_ident");

        let result1 = cache.query(&query1);
        let result2 = cache.query(&query2);

        assert_eq!(result1.len(), 1);
        assert_eq!(result2.len(), 2);

        assert!(result1.contains(&"test_id1".to_string()));
        assert!(result2.contains(&"test_id1".to_string()));
        assert!(result2.contains(&"test_id3".to_string()));
    }

    #[test]
    fn test_remove() {
        let mut cache = CrossMarginPositionsCacheIndexes::new();
        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id2",
            "base",
            "quote",
            "collateral",
            "client_ident2",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id3",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id4",
            "base",
            "quote",
            "collateral",
            "client_ident",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id5",
            "base",
            "quote",
            "collateral",
            "client_ident2",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id6",
            "base",
            "quote",
            "collateral",
            "client_ident3",
            "account_ident",
        ));

        let query1 = CrossMarginPositionsCacheQueryBuilder::new().with_client("client_ident");

        let query2 = CrossMarginPositionsCacheQueryBuilder::new().with_client("client_ident2");

        let query3 = CrossMarginPositionsCacheQueryBuilder::new().with_client("client_ident3");

        let result1 = cache.query(&query1);
        let result2 = cache.query(&query2);
        let result3 = cache.query(&query3);

        assert_eq!(result1.len(), 3);
        assert_eq!(result2.len(), 2);
        assert_eq!(result3.len(), 1);

        assert!(result1.contains(&"test_id1".to_string()));
        assert!(result1.contains(&"test_id3".to_string()));
        assert!(result1.contains(&"test_id4".to_string()));

        assert!(result2.contains(&"test_id2".to_string()));
        assert!(result2.contains(&"test_id5".to_string()));

        assert!(result3.contains(&"test_id6".to_string()));

        cache.remove_index("test_id1");
        cache.remove_index("test_id3");
        cache.remove_index("test_id5");
        cache.remove_index("test_id6");

        let query1 = CrossMarginPositionsCacheQueryBuilder::new().with_client("client_ident");

        let query2 = CrossMarginPositionsCacheQueryBuilder::new().with_client("client_ident2");

        let query3 = CrossMarginPositionsCacheQueryBuilder::new().with_client("client_ident3");

        let result1 = cache.query(&query1);
        let result2 = cache.query(&query2);
        let result3 = cache.query(&query3);

        assert_eq!(result1.len(), 1);
        assert_eq!(result2.len(), 1);
        assert_eq!(result3.len(), 0);

        assert!(result1.contains(&"test_id4".to_string()));
        assert!(result2.contains(&"test_id2".to_string()));
    }

    #[test]
    fn limit_orders_bug_case() {
        let mut cache = CrossMarginPositionsCacheIndexes::new();
        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "EUR",
            "USD",
            "USD",
            "client_ident3",
            "account_ident",
        ));

        let query1 = CrossMarginPositionsCacheQueryBuilder::new()
            .with_base("BTC")
            .with_quote("USD");

        let result1 = cache.query(&query1);

        assert_eq!(0, result1.len());
    }

    #[test]
    fn limit_orders_bug_case2() {
        let mut cache = CrossMarginPositionsCacheIndexes::new();
        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "EUR",
            "USD",
            "USD",
            "client_ident3",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "BTC",
            "USD",
            "USD",
            "client_ident3",
            "account_ident",
        ));

        let query1 = CrossMarginPositionsCacheQueryBuilder::new()
            .with_base("BTC")
            .with_quote("USD");

        let result1 = cache.query(&query1);

        assert_eq!(1, result1.len());
    }

    #[test]
    fn limit_orders_bug_case3() {
        let mut cache = CrossMarginPositionsCacheIndexes::new();
        cache.add_index(&TestIndexStruct::new(
            "test_id1",
            "EUR",
            "USD",
            "USD",
            "client_ident3",
            "account_ident",
        ));

        cache.add_index(&TestIndexStruct::new(
            "test_id2",
            "BTC",
            "USD",
            "USD",
            "client_ident3",
            "account_ident",
        ));

        let query1 = CrossMarginPositionsCacheQueryBuilder::new().with_quote("USD");

        let result1 = cache.query(&query1);

        assert_eq!(2, result1.len());
    }

    #[test]
    fn test_search_by_clienasdt_ident_few() {
        let min: f64 = 10.0;
        let max: f64 = 100.0;

        let target: f64 = min.min(max);
        let target2: f64 = max.min(min);

        println!("{} {}, {:?}", min, max, target);
        println!("{} {}, {:?}", min, max, target2);
    }
}
