use crate::framework::logger::Logger;
use crate::framework::transaction_manager::sql_executor::SQLExecutor;
use crate::framework::transaction_manager::transaction::Transaction;

mod config;
mod sql_executor;
mod transaction;

pub(crate) struct TransactionManager<'a> {
    tx: &'a Transaction,
}

impl<'a> TransactionManager<'a> {
    pub fn new(tx: &'a Transaction) -> Self {
        Self { tx }
    }

    pub fn execute<F>(self, f: F) -> Result<(), ()>
    where
        F: FnOnce(&SQLExecutor) -> Result<(), ()>,
    {
        self.tx.begin();

        let sql_executor = &SQLExecutor::new(self.tx);
        match f(sql_executor) {
            Ok(_) => {
                self.tx.commit();
                Ok(())
            }
            Err(_) => {
                self.tx.rollback();
                Err(())
            }
        }
    }
}
