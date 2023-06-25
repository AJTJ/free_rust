use super::completed_form_dto::CompletedForm;
use actix_web::web;
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{
    actions::get_completed_form_by_id, errors::BigError, graphql_schema::DbPool,
    schema::completed_form_fields,
};

#[derive(Insertable, Debug)]
#[diesel(table_name = completed_form_fields)]
pub struct CompletedFormFieldCreation {
    pub item_order: Option<i32>,

    pub field_name: String,
    pub field_value: Option<String>,
    pub category_name: String,
    pub field_value_type: String,

    pub completed_form_id: Uuid,
    pub user_id: Uuid,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

#[derive(InputObject)]
pub struct CompletedFormFieldInput {
    pub item_order: Option<i32>,
    // field data
    pub field_name: String,
    pub field_value: Option<String>,
    pub category_name: String,
    pub field_value_type: String,

    // relationships
    pub completed_form_id: Uuid,
    pub user_id: Uuid,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Debug, Clone)]
#[graphql(complex)]
pub struct CompletedFormField {
    pub item_order: Option<i32>,
    // field data
    pub field_name: String,
    pub field_value: Option<String>,
    pub category_name: String,
    pub field_value_type: String,
    // relationships
    #[graphql(skip)]
    pub completed_form_id: Uuid,
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
impl CompletedFormField {
    async fn log(&self, ctx: &Context<'_>) -> Result<CompletedForm, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let completed_form_id = self.completed_form_id;
        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_completed_form_by_id(&mut conn, completed_form_id).map(CompletedForm::from)
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }
}
