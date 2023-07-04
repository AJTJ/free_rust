use async_graphql::{OneofObject, Union};
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    sql_types,
};
use serde::{Deserialize, Serialize};

use super::form_v1::form::{FormInputV1, FormOutputV1};

// NOTE: This is only for receiving from the client
#[derive(OneofObject, Debug)]
pub enum FormInput {
    V1(FormInputV1),
}

// All operations are done on this object
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "version")]
#[derive(Union, FromSqlRow)]
pub enum FormOutput {
    V1(FormOutputV1),
}

impl FromSql<sql_types::Jsonb, Pg> for FormOutput {
    fn from_sql(value: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        let bytes = value.as_bytes();
        if bytes[0] != 1 {
            return Err("Unsupported JSONB encoding version".into());
        }
        serde_json::from_slice(&bytes[1..]).map_err(|_| "Invalid Json".into())
    }
}

impl FormOutput {
    pub fn from_input(input: FormInput) -> Self {
        match input {
            FormInput::V1(v1) => FormOutput::V1(FormOutputV1::from(v1)),
        }
    }
}

// impl AsExpression<sql_types::Jsonb> for FormOutput {
//     type Expression;

//     fn as_expression(self) -> Self::Expression {
//         serde_json::to_value(self)
//     }
// }

// impl AsExpression<sql_types::Jsonb> for FormOutput {
//     type Expression;

//     fn as_expression(self) -> Self::Expression {
//         serde_json::to_value(self)
//     }
// }
