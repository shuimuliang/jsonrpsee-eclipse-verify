use crate::entity::{Entity, Model, Column};
use sea_orm::{ConnectionTrait, EntityTrait};
use sea_orm::{DbConn, DbErr};
use sea_query::{Expr, Func, Query as SeaQuery};

pub struct Query;

impl Query {
    #[allow(dead_code)]
    pub async fn find_all(db: &DbConn) -> Result<Vec<Model>, DbErr> {
        Entity::find().all(db).await
    }
    pub async fn find_one(db: &DbConn, id: i64) -> Result<Option<Model>, DbErr> {
        Entity::find_by_id(id).one(db).await
    }

    // max slot id in merkle_tree_proof
    pub async fn find_max_slot(db: &DbConn) -> Result<i64, DbErr> {
        let stmt = SeaQuery::select()
            .expr(Func::max(Expr::col(Column::Slot)))
            .from(Entity).to_owned();
        let builder = db.get_database_backend();
        let query_res = db.query_one(builder.build(&stmt)).await?;

        let res = query_res.unwrap();
        let max_id = res.try_get_by_index(0).unwrap();
        Ok(max_id)
    }
}
