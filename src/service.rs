use std::sync::Arc;

use crate::{Error, ErrorKind, Model, Repository, Result};

pub struct Service<Repos> {
    pub repos: Arc<Repos>,
}
impl<Repos> Service<Repos> {
    pub fn new(repos: Arc<Repos>) -> Self {
        Self { repos }
    }
}
impl<Repos> Service<Repos>
where
    Repos: Repository,
{
    pub async fn query(&self, id: &u32) -> Result<Model> {
        println!("ID:{:?}", id);
        match self.repos.get(id).await {
            Ok(value) => Ok(value),
            Err(e) => {
                println!("Error: {:?}", e.message);
                Err(Error::new(ErrorKind::NotImplement, e.message, e.code))
            }
        }
    }
}

#[cfg(test)]
mod t {
    use crate::repository::MockRepository;

    use super::*;
    #[tokio::test]
    async fn test_query() {
        let mut mock = MockRepository::new();
        mock.expect_get().times(1).return_once(|id| {
            Ok(Model {
                id: *id,
                first_name: "First Name".to_string(),
                last_name: "Last Name".to_string(),
            })
        });
        let service = Service::new(mock.into());
        let result = service.query(&1).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test() {
        assert!(true)
    }
}
