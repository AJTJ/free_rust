use async_graphql::{async_trait::async_trait, Context, Guard, Result};

use crate::{
    actions::get_user_session_data, cookie_helpers::get_cookie_from_token, token_source::Token,
};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Role {
    Admin,
    Guest,
}

struct RoleGuard {
    role: Role,
}

impl RoleGuard {
    fn new(role: Role) -> Self {
        Self { role }
    }
}

#[async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if ctx.data_opt::<Role>() == Some(&self.role) {
            Ok(())
        } else {
            Err("Forbidden".into())
        }
    }
}

pub struct LoggedInGuard {}

// impl LoggedInGuard {
//     fn new() -> Self {
//         Self {}
//     }
// }

#[async_trait]
impl Guard for LoggedInGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if let Some(cookie_data) = get_cookie_from_token(ctx) {
            let user_session = get_user_session_data(ctx, cookie_data.encoded_session_id).await;

            match user_session {
                Ok(u) => Ok(()),
                Err(e) => Err("Forbidden".into()),
            }
        } else {
            Err("Forbidden".into())
        }
    }
}
