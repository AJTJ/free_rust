use crate::{diesel::ExpressionMethods, dto::form_dto::Form};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_form_by_id(conn: &mut PgConnection, form_id_input: Uuid) -> diesel::QueryResult<Form> {
    use crate::schema::forms::dsl::*;

    forms.filter(id.eq(&form_id_input)).get_result::<Form>(conn)
}
