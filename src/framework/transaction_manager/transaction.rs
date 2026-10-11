use crate::framework::logger::Logger;
use rand::random;

pub(crate) struct Transaction {
    logger: Logger,
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            logger: Logger::new(),
        }
    }

    pub fn begin(&self) {
        self.logger.info("[SQL] BEGIN.")
    }

    pub fn rollback(&self) {
        self.logger.info("[SQL] ROLLBACK.")
    }

    pub fn commit(&self) {
        self.logger.info("[SQL] COMMIT.")
    }

    pub fn query(&self, sql: &str) -> Result<(), ()> {
        if random::<f64>() > 0.5 {
            self.logger.info(&format!("[SQL] {}.", sql));
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn execute(&self, sql: &str) -> Result<(), ()> {
        if random::<f64>() > 0.5 {
            self.logger.info(&format!("[SQL] {}.", sql));
            Ok(())
        } else {
            Err(())
        }
    }
}
