use std::collections::HashMap;

use super::{
    CrossMarginActivePosition, CrossMarginPosition, CrossMarginPositionsCacheIndexes,
    CrossMarginPositionsCacheQueryBuilder, CrossMarginPositionsOneOfBulkQueryBuilder,
};

pub struct PositionsCache<T: CrossMarginPosition> {
    pub indexes: CrossMarginPositionsCacheIndexes,
    pub identifier: String,
    pub positions: HashMap<String, T>,
}

impl<T: CrossMarginPosition> PositionsCache<T> {
    pub fn new(identifier: String, positions: Vec<T>) -> Self {
        Self {
            identifier,
            indexes: CrossMarginPositionsCacheIndexes::new(),
            positions: positions
                .into_iter()
                .map(|x| (x.get_id().to_string(), x))
                .collect(),
        }
    }

    pub fn get_by_id(&self, id: &str) -> Option<&T> {
        self.positions.get(id)
    }

    pub fn add_position(&mut self, position: T) {
        metrics::gauge!("cache_positions_amount", "ident" => self.identifier.clone()).increment(1);
        self.indexes.add_index(&position);
        self.positions
            .insert(position.get_id().to_string(), position);
    }

    pub fn remove_position(&mut self, id: &str) -> Option<T> {
        metrics::gauge!("cache_positions_amount", "ident" => self.identifier.clone()).decrement(1);
        self.indexes.remove_index(id);
        self.positions.remove(id)
    }

    pub fn query_positions(&self, query: CrossMarginPositionsCacheQueryBuilder) -> Vec<&T> {
        let indexes = self.indexes.query(&query);
        let mut result = vec![];

        for index in indexes {
            if let Some(position) = self.positions.get(index.as_ref()) {
                result.push(position);
            }
        }

        return result;
    }

    pub fn bulk_query_positions(
        &self,
        query: CrossMarginPositionsOneOfBulkQueryBuilder,
    ) -> Vec<&T> {
        let indexes = self.indexes.bulk_query(&query);
        let mut result = vec![];

        for index in indexes {
            if let Some(position) = self.positions.get(index.as_ref()) {
                result.push(position);
            }
        }

        return result;
    }

    pub fn query_and_select_remove<F>(
        &mut self,
        query: CrossMarginPositionsCacheQueryBuilder,
        is_remove: impl Fn(&T) -> Option<F>,
    ) -> Vec<(T, F)> {
        let indexes = self.indexes.query(&query);

        let mut to_return = vec![];

        for index in indexes {
            if let Some(position) = self.positions.get(index.as_ref()) {
                if let Some(data_to_return) = is_remove(position) {
                    to_return.push((
                        self.remove_position(&position.get_id().to_string())
                            .unwrap(),
                        data_to_return,
                    ));
                }
            }
        }

        metrics::gauge!("cache_positions_amount", "ident" => self.identifier.clone())
            .set(self.positions.len() as f64);

        return to_return;
    }

    pub fn update_position(
        &mut self,
        id: &str,
        update_command: impl Fn(Option<&mut T>) -> Option<T>,
    ) -> Option<T> {
        let position = self.positions.get_mut(id);
        update_command(position)
    }

    pub fn update_positions<F>(
        &mut self,
        query: CrossMarginPositionsCacheQueryBuilder,
        update_command: impl Fn(&mut T) -> Option<F>,
    ) -> Vec<F> {
        let indexes = self.indexes.query(&query);
        let mut result = vec![];
        for index in indexes {
            if let Some(position) = self.positions.get_mut(index.as_ref()) {
                let update_result = update_command(position);
                if let Some(update_result) = update_result {
                    result.push(update_result);
                };
            }
        }

        return result;
    }

    pub fn bulk_update_positions<F>(
        &mut self,
        query: CrossMarginPositionsOneOfBulkQueryBuilder,
        update_command: impl Fn(&mut T) -> Option<F>,
    ) -> Vec<F> {
        let indexes = self.indexes.bulk_query(&query);
        let mut result = vec![];
        for index in indexes {
            if let Some(position) = self.positions.get_mut(index.as_ref()) {
                let update_result = update_command(position);
                if let Some(update_result) = update_result {
                    result.push(update_result);
                };
            }
        }

        return result;
    }
}
