pub trait CrossMarginCacheIndexGenerator {
    fn get_id_index(&self) -> String;
    fn get_base_index(&self) -> Option<String>;
    fn get_quote_index(&self) -> Option<String>;
    fn get_collateral_index(&self) -> Option<String>;
    fn get_client_identification_index(&self) -> Option<String>;
    fn get_account_identification_index(&self) -> Option<String>;
}