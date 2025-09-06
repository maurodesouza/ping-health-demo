use sea_orm::{ entity::prelude::*,  };
use serde::{ Serialize };

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "pings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub service: String,
    pub amount: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

