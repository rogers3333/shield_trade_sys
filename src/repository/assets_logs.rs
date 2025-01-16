//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use std::time::SystemTime;
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "assets_logs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub user_id: String,
    pub symbol: String,
    #[sea_orm(column_type = "Decimal(Some((40, 20)))", nullable)]
    pub before_balance: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((40, 20)))", nullable)]
    pub amount: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((40, 20)))", nullable)]
    pub after_balance: Option<Decimal>,
    pub trans_id: String,
    pub change_type: String,
    pub info: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
