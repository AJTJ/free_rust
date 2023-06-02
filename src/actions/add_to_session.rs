use crate::diesel::ExpressionMethods;
use crate::{data::UserQueryData, session_data::SessionData};

use async_graphql::Context;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn login(
    ctx: &Context<'_>,
    user_id: String,
    session_data: SessionData,
) -> diesel::QueryResult<UserQueryData> {
    let shared_session = ctx.data_unchecked::<Shared<Session>>().clone();
    shared_session.insert(user_id, SessionData).unwrap();

    Ok(user)
}
