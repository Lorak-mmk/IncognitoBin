use anyhow::Result;
use uuid::Uuid;
use crate::db::models::{PasteById, UserById, UserByToken};
use scylla::frame::value::Counter;

#[async_trait::async_trait]
pub trait PasteDbOperations {
    async fn get_user_by_id(&self, userid: Uuid) -> Result<Option<UserById>>;
    async fn get_userid_by_token(&self, token: &str) -> Result<Option<Uuid>>;
    async fn get_paste_by_id(&self, paste_id: Uuid) -> Result<Option<PasteById>>;
    async fn get_pastes_by_userid(&self, userid: Uuid) -> Result<Vec<Uuid>>;
    async fn get_view_count_by_paste_id(&self, paste_id: Uuid) -> Result<Option<Counter>>;
    async fn increment_view_count_by_paste_id(&self, paste_id: Uuid) -> Result<()>;
    async fn insert_user_by_id(&self, user: &UserById) -> Result<()>;
    async fn insert_user_by_token(&self, user: &UserByToken) -> Result<()>;
    async fn insert_paste(&self, paste: &PasteById) -> Result<()>;
    async fn insert_paste_by_user_id(&self, user_id: Uuid, paste_id: Uuid) -> Result<()>;
}