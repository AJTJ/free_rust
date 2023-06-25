use crate::actions::get_form_fields_by_form;
use crate::dive_forms::form_helper::{FormStructure, FormStructureOutput};
use crate::errors::BigError;
use crate::{graphql_schema::DbPool, schema::forms};
use actix_web::web;
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

use super::form_field_dto::FormField;
use super::query_dto::QueryParams;

#[derive(InputObject)]
pub struct FormInput {
    pub form_name: String,
    // If this is an "edit", then include the previous form, or this field if the previous form already has it.
    pub original_form_id: Option<Uuid>,
    // The previous form
    pub previous_form_id: Option<Uuid>,
    pub form_structure: FormStructure,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = forms)]
pub struct FormCreation {
    pub form_name: String,
    pub template_version: Vec<Option<i32>>,

    pub user_id: Uuid,
    pub original_form_id: Option<Uuid>,
    pub previous_form_id: Option<Uuid>,

    // partial default data
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Clone)]
#[graphql(complex)]
pub struct Form {
    pub form_name: String,
    pub template_version: Vec<Option<i32>>,
    // relationship data
    #[graphql(skip)]
    pub user_id: Uuid,
    #[graphql(skip)]
    pub original_form_id: Option<Uuid>,
    #[graphql(skip)]
    pub previous_form_id: Option<Uuid>,

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
impl Form {
    pub async fn form_fields(
        &self,
        ctx: &Context<'_>,
        db_query_dto: Option<QueryParams>,
    ) -> Result<Vec<FormField>, BigError> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

        let logger_id = self.id;
        let user_id = self.user_id;

        web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            get_form_fields_by_form(&mut conn, &logger_id, &user_id, db_query_dto)
                .map(|v| v.into_iter().map(FormField::from).collect())
        })
        .await
        .map_err(|e| BigError::ActixBlockingError { source: e })
        .unwrap()
        .map_err(|e| BigError::DieselQueryError { source: e })
    }
}

#[derive(SimpleObject, Clone)]
pub struct FormOutput {
    pub form: Form,
    pub form_structure: FormStructureOutput,
}
