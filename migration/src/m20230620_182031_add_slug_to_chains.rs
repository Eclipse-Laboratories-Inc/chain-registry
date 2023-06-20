use sea_orm_migration::prelude::*;

use crate::m20230222_234021_create_chain_tables::EvmChain;
use crate::m20230518_213347_add_svm_chains::SvmChain;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let add_slug_alter = Table::alter()
            .table(EvmChain::Table)
            .add_column(ColumnDef::new(Alias::new("slug")).string().unique_key())
            .to_owned();

        let add_svm_slug_alter = Table::alter()
            .table(SvmChain::Table)
            .add_column(ColumnDef::new(Alias::new("slug")).string().unique_key())
            .to_owned();

        manager.alter_table(add_slug_alter).await?;
        manager.alter_table(add_svm_slug_alter).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let drop_slug_alter = Table::alter()
            .table(EvmChain::Table)
            .drop_column(Alias::new("slug"))
            .to_owned();


        let svm_drop_slug_alter = Table::alter()
            .table(SvmChain::Table)
            .drop_column(Alias::new("slug"))
            .to_owned();

        manager.alter_table(drop_slug_alter).await?;
        manager.alter_table(svm_drop_slug_alter).await?;

        Ok(())
    }
}
