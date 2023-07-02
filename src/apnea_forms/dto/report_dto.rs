use crate::{graphql_schema::DbPool, schema::reports};
use actix_web::web;
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use chrono::NaiveDateTime;
use serde_json::Value;
use uuid::Uuid;

#[derive(InputObject)]
pub struct ReportInput {
    pub form_id: Uuid,
    pub original_form_id: Option<Uuid>,
    pub previous_report_id: Option<Uuid>,
    pub session_id: Uuid,
    pub user_id: Uuid,
}

#[derive(InputObject)]
pub struct ReportOutput {
    pub form_id: Uuid,
    pub original_form_id: Option<Uuid>,
    pub previous_report_id: Option<Uuid>,
    pub session_id: Uuid,
    pub user_id: Uuid,
}

impl From<ReportInput> for ReportOutput {
    fn from(value: ReportInput) -> Self {
        ReportOutput {
            form_id: value.form_id,
            original_form_id: value.original_form_id,
            previous_report_id: value.previous_report_id,
            session_id: value.session_id,
            user_id: value.user_id,
        }
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = reports)]
pub struct ReportCreation {
    pub report_version: i32,
    pub report_data: Value,

    pub form_id: Uuid,
    pub original_form_id: Option<Uuid>,
    pub previous_report_id: Option<Uuid>,
    pub session_id: Uuid,
    pub user_id: Uuid,

    // partial default data
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Clone)]
// #[graphql(complex)]
pub struct Report {
    pub report_version: i32,
    pub report_data: Value,
    // relationships
    #[graphql(skip)]
    pub form_id: Uuid,
    #[graphql(skip)]
    pub original_form_id: Option<Uuid>,
    #[graphql(skip)]
    pub previous_report_id: Option<Uuid>,
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

// #[ComplexObject]
// impl Report {
//     async fn report_fields(
//         &self,
//         ctx: &Context<'_>,
//         // db_query_dto: Option<QueryParams>,
//     ) -> Result<Vec<ReportField>, BigError> {
//         let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
//         let log_id = self.id;
//         web::block(move || {
//             let mut conn = pool_ctx.get().unwrap();
//             get_report_fields_by_c_form(&mut conn, &log_id)
//                 .map(|v| v.into_iter().map(ReportField::from).collect())
//         })
//         .await
//         .map_err(|e| BigError::ActixBlockingError { source: e })?
//         .map_err(|e| BigError::DieselQueryError { source: e })
//     }
// }

// #[derive(SimpleObject, Clone)]
// pub struct ReportOutput {
//     pub form: Report,
//     pub form_structure: FormStructureOutput,
// }
