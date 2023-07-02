// use crate::dto::query_dto::QueryParams;
// use crate::errors::BigError;
// use crate::{diesel::ExpressionMethods, dto::form_field_dto::FormField};
// use diesel::{BoolExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
// use uuid::Uuid;

// pub fn get_form_fields_by_form(
//     conn: &mut PgConnection,
//     input_form_id: &Uuid,
//     input_user_id: &Uuid,
//     _db_query_ob: Option<QueryParams>,
// ) -> QueryResult<Vec<FormField>> {
//     use crate::schema::form_fields::dsl::{form_fields, form_id, user_id};

//     form_fields
//         .filter(form_id.eq(&input_form_id).and(user_id.eq(&input_user_id)))
//         .get_results::<FormField>(conn)
// }
