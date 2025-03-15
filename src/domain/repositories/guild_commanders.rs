use crate::domain::value_objects::guild_commander_model::RegisterGuildCommanderModel;
use anyhow::Result;
use axum::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait GuildCommandersRepository {
    async fn register(&self, register_adventerer_model: RegisterGuildCommanderModel)
    -> Result<i32>;
    async fn find_by_username(&self, username: String) -> Result<RegisterGuildCommanderModel>;
}
