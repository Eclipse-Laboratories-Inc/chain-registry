use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum EvmChain {
    Table,
    RpcUrls,
    #[iden = "chainId"] // Renaming the identifier
    ChainId,
    ChainName,
    NativeCurrencyName,
    NativeCurrencyDecimals,
    NativeCurrencySymbol,
    BlockExplorerUrls,
    IconUrls,
    DataAvailability,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = Table::create();
        table
            .table(EvmChain::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(EvmChain::ChainId)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(
                ColumnDef::new(EvmChain::RpcUrls)
                    .array(ColumnType::String(Some(255)))
                    .not_null(),
            )
            .col(
                ColumnDef::new(EvmChain::BlockExplorerUrls)
                    .array(ColumnType::String(Some(255)))
                    .not_null(),
            )
            .col(
                ColumnDef::new(EvmChain::IconUrls)
                    .array(ColumnType::String(Some(255)))
                    .not_null(),
            )
            .col(ColumnDef::new(EvmChain::ChainName).string().not_null())
            .col(
                ColumnDef::new(EvmChain::NativeCurrencyName)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(EvmChain::NativeCurrencyDecimals)
                    .integer()
                    .not_null(),
            )
            .col(
                ColumnDef::new(EvmChain::NativeCurrencySymbol)
                    .string()
                    .not_null(),
            )
            .col(
                ColumnDef::new(EvmChain::DataAvailability)
                    .string()
                    .not_null(),
            );
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut table = Table::drop();
        table.table(EvmChain::Table);
        manager.drop_table(table).await
    }
}
