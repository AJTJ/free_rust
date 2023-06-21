use crate::{diesel::ExpressionMethods, dto::completed_form_dto::CompletedForm};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_completed_form_by_id(
    conn: &mut PgConnection,
    log_id_input: Uuid,
) -> diesel::QueryResult<CompletedForm> {
    use crate::schema::completed_forms::dsl::*;

    completed_forms
        .filter(id.eq(&log_id_input))
        .get_result::<CompletedForm>(conn)
}
