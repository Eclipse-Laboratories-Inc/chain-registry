use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum EvmChain {
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
    DataAvailability
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(EvmChain::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EvmChain::ChainId)
                            .string()
                            .not_null()
                    )
                    .col(ColumnDef::new(EvmChain::RpcUrls).array(ColumnType::String(Some(255))).not_null())
                    .col(ColumnDef::new(EvmChain::BlockExplorerUrls).array(ColumnType::String(Some(255))).not_null())
                    .col(ColumnDef::new(EvmChain::IconUrls).array(ColumnType::String(Some(255))).not_null())
                    .col(ColumnDef::new(EvmChain::ChainName).string().not_null())
                    .col(ColumnDef::new(EvmChain::NativeCurrencyName).string().not_null())
                    .col(ColumnDef::new(EvmChain::NativeCurrencyDecimals).integer().not_null())
                    .col(ColumnDef::new(EvmChain::NativeCurrencySymbol).string().not_null())
                    .col(ColumnDef::new(EvmChain::DataAvailability).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EvmChain::Table).to_owned())
            .await
    }
}

