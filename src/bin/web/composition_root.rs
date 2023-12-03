use community::{
    adapters::repositories::SqlRepository,
    domains::account::commands::{CreateAccount, SignInAccount},
    services::{
        account::handlers::AccountHandler,
        responses::{ApplicationResponse, ServiceError},
    },
};

pub struct CompositionRoot<T>(pub T);
impl CompositionRoot<CreateAccount> {
    pub async fn sign_up_account(self) -> Result<ApplicationResponse, ServiceError> {
        let mut handler = AccountHandler::new(SqlRepository::new().await);
        handler.sign_up_account(self.0).await
    }
}

impl CompositionRoot<SignInAccount> {
    pub async fn sign_in_account(self) -> Result<ApplicationResponse, ServiceError> {
        let handler = AccountHandler::new(SqlRepository::new().await);
        handler.sign_in_account(self.0).await
    }
}

impl<T> From<T> for CompositionRoot<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
