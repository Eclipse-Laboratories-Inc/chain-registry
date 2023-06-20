use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum SvmChain {
    Table,
    RpcUrls,
    #[iden = "chainName"] // Renaming the identifier
    ChainName,
    BlockExplorerUrls,
    DataAvailability,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = Table::create();
        table
            .table(SvmChain::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SvmChain::ChainName)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(
                ColumnDef::new(SvmChain::RpcUrls)
                    .array(ColumnType::String(Some(255)))
                    .not_null(),
            )
            .col(
                ColumnDef::new(SvmChain::BlockExplorerUrls)
                    .array(ColumnType::String(Some(255)))
                    .not_null(),
            )
           
            .col(
                ColumnDef::new(SvmChain::DataAvailability)
                    .string()
                    .not_null(),
            );
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = Table::drop();
        table.table(SvmChain::Table);
        manager.drop_table(table).await
    }
}
