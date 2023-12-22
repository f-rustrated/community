use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Debug, Serialize, Deserialize, PartialEq, Default)]
#[sqlx(type_name = "account_status", rename_all = "lowercase")]
pub enum AccountStatus {
    #[default]
    Active,
    Deleted,
    Abnormal,
}
