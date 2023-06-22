use super::completed_form_dto::CompletedForm;
use actix_web::web;
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject, ID};
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
    pub completed_form_id: ID,
    pub user_id: ID,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Debug)]
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
    #[graphql(derived(into = "ID"))]
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
        .map_err(|e| BigError::BlockingError { source: e })?
        .map_err(|e| BigError::DieselQueryError { source: e })
    }
}

// impl From<LogEntry> for LogEntryOutput {
//     fn from(x: LogEntry) -> Self {
//         LogEntryOutput {
//             item_order: x.item_order,
//             category_type: x.category_type,
//             input_type: x.input_type,
//             input_value: x.input_value,
//             completed_form_id: x.completed_form_id,
//             user_id: x.user_id,
//             id: x.id,
//             created_at: x.created_at,
//             updated_at: x.updated_at,
//             is_active: x.is_active,
//             archived_at: x.archived_at,
//             archived_by: x.archived_by,
//         }
//     }
// }

// #[derive(SimpleObject)]
// #[graphql(complex)]
// pub struct LogEntryOutput {
//     pub item_order: Option<i32>,
//     pub category_type: String,
//     pub input_type: String,
//     pub input_value: Option<String>,

//     #[graphql(skip)]
//     pub completed_form_id: Uuid,
//     #[graphql(skip)]
//     pub user_id: Uuid,

//     pub id: Uuid,
//     pub created_at: NaiveDateTime,
//     pub updated_at: NaiveDateTime,
//     pub is_active: bool,
//     pub archived_at: Option<NaiveDateTime>,
//     pub archived_by: Option<Uuid>,
// }

// #[ComplexObject]
// impl LogEntryOutput {
//     async fn log(&self, ctx: &Context<'_>) -> Result<CompletedFormOutput, BigError> {
//         let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

//         let completed_form_id = self.completed_form_id;
//         web::block(move || {
//             let mut conn = pool_ctx.get().unwrap();
//             get_log_by_id(&mut conn, completed_form_id).map(CompletedFormOutput::from)
//         })
//         .await
//         .map_err(|e| BigError::BlockingError { source: e })?
//         .map_err(|e| BigError::DieselQueryError { source: e })
//     }
// }
