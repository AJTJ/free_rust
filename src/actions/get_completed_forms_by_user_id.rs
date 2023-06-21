use crate::diesel::ExpressionMethods;
use crate::dto::completed_form_dto::CompletedForm;
use crate::dto::query_dto::QueryParams;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_completed_forms_by_user_id(
    conn: &mut PgConnection,
    input_user_id: Uuid,
    db_query_ob: Option<QueryParams>,
) -> diesel::QueryResult<Vec<CompletedForm>> {
    use crate::schema::completed_forms::dsl::{completed_forms, user_id};

    completed_forms
        .filter(user_id.eq(&input_user_id))
        .limit(db_query_ob.and_then(|q| q.limit).unwrap_or(100) as i64)
        .get_results::<CompletedForm>(conn)
}
