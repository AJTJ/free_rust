use crate::{
    auth::dto::user_dto::{User, UserRetrievalData},
    diesel::ExpressionMethods,
};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

pub fn get_user(conn: &mut PgConnection, input: UserRetrievalData) -> diesel::QueryResult<User> {
    use crate::schema::users::dsl::{email, id as user_id, users};

    match input {
        UserRetrievalData::Email(e) => users.filter(email.eq(&e)).first::<User>(conn),
        UserRetrievalData::Id(id) => users.filter(user_id.eq(&id)).first::<User>(conn),
    }
}
