use crate::dto::query_dto::QueryParams;
use crate::{diesel::ExpressionMethods, dto::form_dto::Form};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_forms_by_user_id(
    conn: &mut PgConnection,
    input_user_id: Uuid,
    db_query_ob: Option<QueryParams>,
) -> diesel::QueryResult<Vec<Form>> {
    use crate::schema::forms::dsl::{forms, user_id};

    forms
        .filter(user_id.eq(&input_user_id))
        .limit(db_query_ob.and_then(|q| q.limit).unwrap_or(100) as i64)
        .get_results::<Form>(conn)
}
