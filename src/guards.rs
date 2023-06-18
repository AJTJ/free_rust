use async_graphql::{async_trait::async_trait, Context, Guard, Result};
use tracing::info;

use crate::{
    actions::get_user_session_data, env_data::DEV_ENV,
    helpers::cookie_helpers::get_cookie_from_token, token_source::Token, SharedVars,
};

#[derive(Eq, PartialEq, Copy, Clone)]
// enum Role {
//     Admin,
//     Guest,
// }

// struct RoleGuard {
//     role: Role,
// }

// impl RoleGuard {
//     fn new(role: Role) -> Self {
//         Self { role }
//     }
// }

// #[async_trait]
// impl Guard for RoleGuard {
//     async fn check(&self, ctx: &Context<'_>) -> Result<()> {
//         if ctx.data_opt::<Role>() == Some(&self.role) {
//             Ok(())
//         } else {
//             Err("Forbidden".into())
//         }
//     }
// }
#[derive(Default)]
pub struct LoggedInGuard {}

impl LoggedInGuard {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Guard for LoggedInGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        // PURELY A WORKAROUND FOR GRAPHQL PLAYGROUND
        let shared_vars = ctx.data_unchecked::<SharedVars>();
        if shared_vars.environment == DEV_ENV {
            let token = ctx.data::<Token>();
            match token {
                Ok(token) => {
                    if token.0 == "LOGGED" {
                        info!("workaround works");
                        return Ok(());
                    }
                }
                Err(e) => {}
            }
        }

        match get_cookie_from_token(ctx) {
            Ok(c) => {
                match get_user_session_data(ctx, c.encoded_session_id).await {
                    Ok(s) => {
                        info!("The session data in guard: {s:?}");
                        // TODO Need to check that the token hasn't expired
                        // TODO Should I extend the token's lifetime if it hasn't expired?
                        Ok(())
                    }
                    Err(e) => {
                        info!("RedisError: {}", e);
                        Err(e.into())
                    }
                }
            }
            Err(e) => {
                info!("no cookie: {}", e);
                Err(e.into())
            }
        }
    }
}
