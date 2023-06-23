use crate::{
    actions::get_log_entries_by_log,
    dive_forms::form_helper::{FormStructure, FormStructureOutput},
    errors::BigError,
    graphql_schema::DbPool,
    schema::completed_forms,
};
use actix_web::web;
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::{completed_form_field_dto::CompletedFormField, query_dto::QueryParams};

#[derive(InputObject)]
pub struct CompletedFormInput {
    pub completed_form_name: String,
    pub form_structure: FormStructure,

    pub form_id: Uuid,
    pub original_form_id: Option<Uuid>,
    pub previous_completed_form_id: Option<Uuid>,
    pub session_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = completed_forms)]
pub struct CompletedFormCreation {
    pub completed_form_name: String,

    pub form_id: Uuid,
    pub original_form_id: Option<Uuid>,
    pub previous_completed_form_id: Option<Uuid>,
    pub session_id: Uuid,
    pub user_id: Uuid,

    // partial default data
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Debug)]
// #[graphql(complex)]
pub struct CompletedForm {
    pub completed_form_name: Option<String>,
    pub template_version: Vec<Option<i32>>,
    // relationships
    #[graphql(skip)]
    pub form_id: Uuid,
    #[graphql(skip)]
    pub original_form_id: Option<Uuid>,
    #[graphql(skip)]
    pub previous_completed_form_id: Option<Uuid>,
    #[graphql(skip)]
    pub session_id: Uuid,
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
impl CompletedForm {
    async fn completed_form_fields(
        &self,
        ctx: &Context<'_>,
        db_query_dto: Option<QueryParams>,
    ) -> Result<Vec<CompletedFormField>, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let log_id = self.id;

        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_log_entries_by_log(&mut conn, &log_id, db_query_dto)
                .map(|v| v.into_iter().map(CompletedFormField::from).collect())
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }
}

#[derive(SimpleObject)]
pub struct CompletedFormOutput {
    pub form: CompletedForm,
    pub fields: Vec<CompletedFormField>,
    pub form_structure: FormStructureOutput,
}
