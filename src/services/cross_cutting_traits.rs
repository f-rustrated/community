use super::responses::BaseError;

pub trait TransactionUnitOfWork {
    fn begin(&mut self) -> impl std::future::Future<Output = Result<(), BaseError>> + Send;
    fn commit(&mut self) -> impl std::future::Future<Output = Result<(), BaseError>> + Send;
    fn rollback(&mut self) -> impl std::future::Future<Output = Result<(), BaseError>> + Send;
}
