use crate::{apnea_forms::dto::form_dto::Form, diesel::ExpressionMethods};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_form(conn: &mut PgConnection, form_id_input: Uuid) -> diesel::QueryResult<Form> {
    use crate::schema::forms::dsl::*;

    forms.filter(id.eq(&form_id_input)).get_result::<Form>(conn)
}
