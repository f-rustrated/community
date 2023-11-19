use async_trait::async_trait;

#[async_trait]
pub trait TransactionUnitOfWork {
    async fn begin(&self);
    async fn commit(&self);
    async fn rollback(&self);
}
