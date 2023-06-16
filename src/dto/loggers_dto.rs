use crate::schema::sql_types::PredefinedInputTypes;
use async_graphql::{Enum, OutputType, SimpleObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

// This one needs to match 1:1
#[derive(Queryable, SimpleObject, Debug)]
// #[graphql(complex)]
pub struct LoggerQueryData {
    pub logger_name: String,
    pub user_id: Uuid,
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

// impl LoggerQueryData {
//     async fn logger_entries(
//         &self,
//         ctx: &Context<'_>,
//         db_query_dto: Option<DBQueryObject>,
//     ) -> FieldResult<Vec<LoggerEntryQueryData>> {
//         let pool_ctx = ctx.data_unchecked::<DbPool>().clone();

//         let logger_id = self.id;

//         web::block(move || {
//             let mut conn = pool_ctx.get().unwrap();
//             get_logger_entries_by_user(&mut conn, &logger_id, db_query_dto)
//         })
//         .await
//         .expect("error in logger_entries web::block")
//         .expect("error in logger_entries")
//     }
// }

#[derive(SimpleObject)]
// #[graphql(complex)]
pub struct LoggerQueryDataOutput {
    pub logger_name: String,
    pub user_id: Uuid,
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
}

// impl LoggerQueryDataOutput {
//     async fn logger_entries() {};
// }

// LOGGER ENTRIES

// #[derive(SqlType)]
// #[diesel(postgres_type(name = "PredefinedInputTypes"))]
// pub struct InputTypes;

// #[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
// #[diesel(sql_type = PredefinedInputTypes)]
// pub enum InputTypesEnum {
//     INTEGER,
//     ENUM,
//     INTERVAL,
//     TIMESTAMP,
//     TEXT,
// }

// #[derive(diesel_derive_enum::DbEnum, Debug, Enum)]
// #[ExistingTypePath = "crate::schema::sql_types::PredefinedInputTypes"]
// #[graphql(remote = "remote_crate::RemoteEnum")]
#[derive(Debug)]
pub enum InputTypesEnum {
    INTEGER,
    ENUM,
    INTERVAL,
    TIMESTAMP,
    TEXT,
}

// This one needs to match 1:1
#[derive(Queryable, SimpleObject)]
// #[graphql(complex)]
pub struct LoggerEntryQueryData {
    pub item_order: Option<i32>,
    pub field_name: String,
    pub category_name: String,
    pub input_type: String,
    pub logger_id: Uuid,
    pub user_id: Uuid,

    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_active: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

// #[derive(SimpleObject)]
// pub struct LoggerEntryQueryDataOutput {
//     pub item_order: Option<i32>,
//     pub field_name: String,
//     pub category_name: String,
//     pub input_type: String,
//     pub logger_id: Uuid,
//     pub user_id: Uuid,

//     pub id: Uuid,
//     pub created_at: NaiveDateTime,
//     pub updated_at: NaiveDateTime,
//     pub is_active: bool,
//     pub deleted_at: Option<NaiveDateTime>,
//     pub deleted_by: Option<Uuid>,
// }
