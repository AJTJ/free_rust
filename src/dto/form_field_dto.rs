use crate::{
    actions::get_form_by_id, errors::BigError, graphql_schema::DbPool, schema::form_fields,
};
use actix_web::web;
use async_graphql::{ComplexObject, Context, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::form_dto::Form;

#[derive(Insertable, Debug)]
#[diesel(table_name = form_fields)]
pub struct FormFieldCreation {
    pub field_order: Option<i32>,

    pub field_name: String,
    pub field_value: Option<Vec<String>>,
    pub category_name: String,
    pub field_value_type: Vec<String>,

    pub form_id: Uuid,
    pub user_id: Uuid,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Clone)]
#[graphql(complex)]
pub struct FormField {
    pub field_order: Option<i32>,
    // field data
    pub field_name: String,
    pub field_value: Option<Vec<Option<String>>>,
    pub category_name: String,
    pub field_value_type: Vec<Option<String>>,
    // relationships
    #[graphql(skip)]
    pub form_id: Uuid,
    #[graphql(skip)]
    pub user_id: Uuid,
    // default data
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    #[graphql(skip)]
    pub archived_at: Option<NaiveDateTime>,
    #[graphql(skip)]
    pub archived_by: Option<Uuid>,
}

#[ComplexObject]
impl FormField {
    async fn form(&self, ctx: &Context<'_>) -> Result<Form, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let form_id = self.form_id;
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_form_by_id(&mut conn, form_id).map(Form::from)
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }
}
