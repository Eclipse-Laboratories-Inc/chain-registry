pub use sea_orm_migration::prelude::*;

mod m20230222_234021_create_chain_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230222_234021_create_chain_tables::Migration)]
    }
}
