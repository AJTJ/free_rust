use async_graphql::{async_trait::async_trait, Context, Guard, Result};
use tracing::info;

use crate::{
    auth::{actions::get_user_session, utility::token_helpers::get_cookie_from_token},
    env_data::DEV_ENV,
    SharedEnvVars,
};

#[derive(Eq, PartialEq, Copy, Clone, Default)]
pub struct LoggedInGuard {}

impl LoggedInGuard {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Guard for LoggedInGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        match get_cookie_from_token(ctx) {
            Ok(c) => {
                // info!("The cookie struct in guard: {c:?}");

                if let Some(session_id) = c.encoded_session_id {
                    match get_user_session(ctx, session_id).await {
                        Ok(s) => {
                            // info!("The session data in guard: {s:?}");
                            // TODO Need to check that the token hasn't expired
                            // TODO Should I extend the token's lifetime if it hasn't expired?
                            Ok(())
                        }
                        Err(e) => {
                            info!("RedisError: {}", e);
                            Err(e.into())
                        }
                    }
                } else {
                    Err("no session id".into())
                }
            }
            Err(e) => {
                info!("no cookie: {}", e);
                Err(e.into())
            }
        }
    }
}

#[derive(Default)]
pub struct DevelopmentGuard {}

impl DevelopmentGuard {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Guard for DevelopmentGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let shared_vars = ctx.data_unchecked::<SharedEnvVars>();
        if shared_vars.environment == DEV_ENV {
            Ok(())
        } else {
            Err("Not in dev".into())
        }
    }
}

#[derive(Default)]
pub struct NoAllow {}

impl NoAllow {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Guard for NoAllow {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        Err("".into())
    }
}
