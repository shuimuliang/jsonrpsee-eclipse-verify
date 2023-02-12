use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "merkle_tree_proof", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, name="slot")]
    pub slot: i64,
    pub root_hash: String,
    #[sea_orm(ignore, default = "1")]
    pub verify_status: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
