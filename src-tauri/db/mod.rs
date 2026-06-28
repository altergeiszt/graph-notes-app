pub async fn run(db: &Surreal<SurrealKv>) -> Result<(), surrealdb::Error> {
    db.query(include_str!("schema.surql")).await?;
    Ok(())
}