use axum::async_trait;

use crate::{Model, Result};

#[mockall::automock]
#[async_trait]
pub trait Repository {
    async fn get(&self, id: &u32) -> Result<Model>;
}
