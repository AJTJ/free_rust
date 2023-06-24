use crate::dto::query_dto::QueryParams;
use crate::{diesel::ExpressionMethods, dto::completed_form_field_dto::CompletedFormField};
use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use uuid::Uuid;

pub fn get_completed_form_fields_by_c_form(
    conn: &mut PgConnection,
    input_completed_form_id: &Uuid,
) -> QueryResult<Vec<CompletedFormField>> {
    use crate::schema::completed_form_fields::dsl::{completed_form_fields, completed_form_id};

    completed_form_fields
        .filter(completed_form_id.eq(&input_completed_form_id))
        .get_results::<CompletedFormField>(conn)
}
