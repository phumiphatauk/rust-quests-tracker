use std::sync::Arc;

use crate::{
    domain::repositories::{
        adventurers::AdventurersRepository, guild_commanders::GuildCommandersRepository,
    },
    infrastructure::jwt_authentication::authentication_model::LoginModel,
};

pub struct AuthenticationUseCase<T1, T2>
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    authentication_repository: Arc<T1>,
    guild_commanders_repository: Arc<T2>,
}

impl<T1, T2> AuthenticationUseCase<T1, T2>
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    pub fn new(authentication_repository: Arc<T1>, guild_commanders_repository: Arc<T2>) -> Self {
        Self {
            authentication_repository,
            guild_commanders_repository,
        }
    }

    pub async fn adventurers_login(&self, login_model: LoginModel) {
        unimplemented!()
    }

    pub async fn adventurers_refresh_token(&self, refresh_token: String) {
        unimplemented!()
    }

    pub async fn guild_commanders_login(&self, login_model: LoginModel) {
        unimplemented!()
    }

    pub async fn guild_commanders_refresh_token(&self, refresh_token: String) {
        unimplemented!()
    }
}
