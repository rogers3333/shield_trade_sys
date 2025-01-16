//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "trade_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uuid: String,
    pub base: String,
    pub symbol: String,
    pub trade_id: String,
    pub ask: String,
    pub bid: String,
    pub trade_by: i16,
    pub ask_uid: String,
    pub bid_uid: String,
    #[sea_orm(column_type = "Decimal(Some((40, 20)))")]
    pub price: Decimal,
    #[sea_orm(column_type = "Decimal(Some((40, 20)))")]
    pub quantity: Decimal,
    #[sea_orm(column_type = "Decimal(Some((40, 20)))")]
    pub amount: Decimal,
    #[sea_orm(column_type = "Decimal(Some((40, 20)))")]
    pub ask_fee_rate: Decimal,
    #[sea_orm(column_type = "Decimal(Some((40, 20)))")]
    pub ask_fee: Decimal,
    #[sea_orm(column_type = "Decimal(Some((40, 20)))")]
    pub bid_fee_rate: Decimal,
    #[sea_orm(column_type = "Decimal(Some((40, 20)))")]
    pub bid_fee: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
