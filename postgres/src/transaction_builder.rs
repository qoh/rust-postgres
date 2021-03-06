use crate::{Error, IsolationLevel, Transaction};
use tokio::runtime::Runtime;

/// A builder for database transactions.
pub struct TransactionBuilder<'a> {
    runtime: &'a mut Runtime,
    builder: tokio_postgres::TransactionBuilder<'a>,
}

impl<'a> TransactionBuilder<'a> {
    pub(crate) fn new(
        runtime: &'a mut Runtime,
        builder: tokio_postgres::TransactionBuilder<'a>,
    ) -> TransactionBuilder<'a> {
        TransactionBuilder { runtime, builder }
    }

    /// Sets the isolation level of the transaction.
    pub fn isolation_level(mut self, isolation_level: IsolationLevel) -> Self {
        self.builder = self.builder.isolation_level(isolation_level);
        self
    }

    /// Sets the transaction to read-only.
    pub fn read_only(mut self) -> Self {
        self.builder = self.builder.read_only();
        self
    }

    /// Sets the transaction to be deferrable.
    ///
    /// If the transaction is also serializable and read only, creation of the transaction may block, but when it
    /// completes the transaction is able to run with less overhead and a guarantee that it will not be aborted due to
    /// serialization failure.
    pub fn deferrable(mut self) -> Self {
        self.builder = self.builder.deferrable();
        self
    }

    /// Begins the transaction.
    ///
    /// The transaction will roll back by default - use the `commit` method to commit it.
    pub fn start(self) -> Result<Transaction<'a>, Error> {
        let transaction = self.runtime.block_on(self.builder.start())?;
        Ok(Transaction::new(self.runtime, transaction))
    }
}
