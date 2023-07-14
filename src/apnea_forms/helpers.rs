use async_graphql::{OneofObject, Union};
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    pg::{Pg, PgValue},
    sql_types,
};
use serde::{Deserialize, Serialize};

use super::form_v1::form::FormV1;

// NOTE: This is only for receiving from the client
#[derive(OneofObject, Debug)]
pub enum FormRequest {
    V1(FormV1),
}

// All operations are done on this object
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "version")]
#[derive(Union, FromSqlRow)]
pub enum FormResponse {
    V1(FormV1),
}

impl FromSql<sql_types::Jsonb, Pg> for FormResponse {
    fn from_sql(value: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        let bytes = value.as_bytes();
        if bytes[0] != 1 {
            return Err("Unsupported JSONB encoding version".into());
        }
        serde_json::from_slice(&bytes[1..])
            .map_err(|_| "Invalid FormResponse Json: bad migration".into())
    }
}

impl FormResponse {
    pub fn from_input(input: FormRequest) -> Self {
        match input {
            FormRequest::V1(v1) => FormResponse::V1(FormV1::from(v1)),
        }
    }
}
