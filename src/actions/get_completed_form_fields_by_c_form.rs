use crate::dto::query_dto::QueryParams;
use crate::{diesel::ExpressionMethods, dto::completed_form_field_dto::CompletedFormField};
use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use uuid::Uuid;

pub fn get_log_entries_by_log(
    conn: &mut PgConnection,
    input_completed_form_id: &Uuid,
    db_query_ob: Option<QueryParams>,
) -> QueryResult<Vec<CompletedFormField>> {
    use crate::schema::completed_form_fields::dsl::{completed_form_fields, completed_form_id};

    completed_form_fields
        .filter(completed_form_id.eq(&input_completed_form_id))
        .limit(db_query_ob.and_then(|q| q.limit).unwrap_or(1000) as i64)
        .get_results::<CompletedFormField>(conn)
}
