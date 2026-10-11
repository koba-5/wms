use crate::framework::transaction_manager::transaction::Transaction;

pub struct SQLExecutor<'a> {
    tx: &'a Transaction,
}

impl<'a> SQLExecutor<'a> {
    pub fn new(tx: &'a Transaction) -> Self {
        Self { tx }
    }

    pub fn query(&self, sql: &str) -> Result<(), ()> {
        self.tx.query(sql)
    }

    pub fn execute(&self, sql: &str) -> Result<(), ()> {
        self.tx.execute(sql)
    }
}
