#[derive(Debug, Clone)]
pub struct CrossMarginPositionsCacheQueryBuilder {
    pub base: Option<String>,
    pub quote: Option<String>,
    pub collateral: Option<String>,
    pub client: Option<String>,
    pub account: Option<String>,
}

impl CrossMarginPositionsCacheQueryBuilder {
    pub fn new() -> Self {
        Self {
            base: None,
            quote: None,
            collateral: None,
            client: None,
            account: None,
        }
    }

    pub fn with_base(mut self, base: &str) -> Self {
        self.base = Some(base.to_string());
        self
    }

    pub fn with_quote(mut self, quote: &str) -> Self {
        self.quote = Some(quote.to_string());
        self
    }

    pub fn with_collateral(mut self, collateral: &str) -> Self {
        self.collateral = Some(collateral.to_string());
        self
    }
    pub fn with_client(mut self, client_ident: &str) -> Self {
        self.client = Some(client_ident.to_string());
        self
    }
    pub fn with_account(mut self, account_ident: &str) -> Self {
        self.account = Some(account_ident.to_string());
        self
    }

    pub fn filters_count(&self) -> usize {
        let mut count = 0;
        if self.base.is_some() {
            count += 1;
        }
        if self.quote.is_some() {
            count += 1;
        }
        if self.collateral.is_some() {
            count += 1;
        }
        if self.client.is_some() {
            count += 1;
        }
        if self.account.is_some() {
            count += 1;
        }
        count
    }
}

#[derive(Debug, Clone)]
pub struct CrossMarginPositionsOneOfBulkQueryBuilder {
    pub base: Option<Vec<String>>,
    pub quote: Option<Vec<String>>,
    pub collateral: Option<Vec<String>>,
    pub client: Option<Vec<String>>,
    pub account: Option<Vec<String>>,
}

impl CrossMarginPositionsOneOfBulkQueryBuilder {
    pub fn new() -> Self {
        Self {
            base: None,
            quote: None,
            collateral: None,
            client: None,
            account: None,
        }
    }

    pub fn with_base(mut self, base: Vec<String>) -> Self {
        self.base = Some(base);
        self
    }

    pub fn with_quote(mut self, quote: Vec<String>) -> Self {
        self.quote = Some(quote);
        self
    }

    pub fn with_collateral(mut self, collateral: Vec<String>) -> Self {
        self.collateral = Some(collateral);
        self
    }
    pub fn with_client(mut self, client_ident: Vec<String>) -> Self {
        self.client = Some(client_ident);
        self
    }
    pub fn with_account(mut self, account_ident: Vec<String>) -> Self {
        self.account = Some(account_ident);
        self
    }

    pub fn filters_count(&self) -> usize {
        let mut count = 0;
        if self.base.is_some() {
            count += 1;
        }
        if self.quote.is_some() {
            count += 1;
        }
        if self.collateral.is_some() {
            count += 1;
        }
        if self.client.is_some() {
            count += 1;
        }
        if self.account.is_some() {
            count += 1;
        }
        count
    }
}
