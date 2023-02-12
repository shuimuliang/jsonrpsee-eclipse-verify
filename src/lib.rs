use anyhow::anyhow;
use jsonrpsee::core::{async_trait, RpcResult};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::types::error::CallError;
use sea_orm::DatabaseConnection;
use std::fmt::Display;

mod entity;
mod query;

pub struct RpcImpl {
    pub conn: DatabaseConnection,
}

impl RpcImpl {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[rpc(server)]
trait PostRpc {
    #[method(name = "ping")]
    async fn ping(&self) -> RpcResult<String>;

    #[method(name = "get_slot_height")]
    async fn get_slot_height(&self) -> RpcResult<i64>;

    #[method(name = "get_slot_status")]
    async fn get_slot_status(&self, slot: i64) -> RpcResult<Option<entity::Model>>;
}

#[async_trait]
impl PostRpcServer for RpcImpl {
    async fn ping(&self) -> RpcResult<String> {
        Ok("pong".to_owned())
    }
    async fn get_slot_height(&self) -> RpcResult<i64> {
        query::Query::find_max_slot(&self.conn)
            .await
            .internal_call_error()
    }

    async fn get_slot_status(&self, slot: i64) -> RpcResult<Option<entity::Model>> {
        query::Query::find_one(&self.conn,  slot)
            .await
            .internal_call_error()
    }
}

trait IntoJsonRpcResult<T> {
    fn internal_call_error(self) -> RpcResult<T>;
}

impl<T, E> IntoJsonRpcResult<T> for Result<T, E>
where
    E: Display,
{
    fn internal_call_error(self) -> RpcResult<T> {
        self.map_err(|e| jsonrpsee::core::Error::Call(CallError::Failed(anyhow!("{}", e))))
    }
}
