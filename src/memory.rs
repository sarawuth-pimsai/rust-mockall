use axum::async_trait;

use crate::{Model, Repository, Result};

pub struct Memory;

#[async_trait]
impl Repository for Memory {
    async fn get(&self, id: &u32) -> Result<Model> {
        let model = Model {
            id: *id,
            first_name: "First Name".to_string(),
            last_name: "Last Name".to_string(),
        };
        Ok(model)
    }
}
