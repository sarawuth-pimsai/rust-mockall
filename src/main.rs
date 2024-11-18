use std::sync::Arc;

use mock::{Memory, Result, Service};
#[tokio::main]
async fn main() -> Result<()> {
    let repos = Arc::new(Memory {});
    let service = Service::new(repos);
    let result = service.query(&1).await?;
    println!("{:#?}", result);
    println!("Hello, world!");
    Ok(())
}
