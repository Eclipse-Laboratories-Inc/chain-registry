pub use sea_orm_migration::prelude::*;

mod m20230222_234021_create_chain_tables;
mod m20230518_213347_add_svm_chains;
mod m20230620_182031_add_slug_to_chains;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230222_234021_create_chain_tables::Migration),
            Box::new(m20230518_213347_add_svm_chains::Migration),
            Box::new(m20230620_182031_add_slug_to_chains::Migration),
        ]

    }
}
