use community::adapters::repositories::SqlRepository;

#[tokio::main]
async fn main() {
    let database_url = "postgres://frustacean:abc123@localhost:5434/community";
    let repository = match SqlRepository::new(database_url).await {
        Ok(repository) => repository,
        Err(err) => {
            eprintln!("crate SqlRepository failed: {:?}", err);
            return;
        }
    };

    if let Err(e) = repository.perform_query().await {
        eprintln!("Error performing query: {:?}", e);
    }

    println!("hello world")
}
