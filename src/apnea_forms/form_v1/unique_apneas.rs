use async_graphql::{Context, InputObject, OneofObject, SimpleObject, Union};
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    pg::{Pg, PgValue},
    sql_types,
};
use serde::{Deserialize, Serialize};

use super::{
    deep_dive::DeepDiveReportFieldsV1, dynamic::DynamicReportFieldsV1,
    static_activity::StaticReportFieldsV1,
};

// REQUEST

#[derive(OneofObject, Serialize, Deserialize, Clone)]
pub enum UniqueApneaActivityRequest {
    DeepDiveV1(DeepDiveReportFieldsV1),
    DynDiveV1(DynamicReportFieldsV1),
    StaticHoldsV1(StaticReportFieldsV1),
}

// #[derive(Serialize, Deserialize, InputObject, Clone)]
// pub struct UniqueApneaActivitiesRequest {
//     pub activities: Vec<UniqueApneaActivityRequest>,
// }

// RESPONSE

#[derive(Union, Serialize, Deserialize, Clone, FromSqlRow)]
pub enum UniqueApneaActivity {
    DeepDiveV1(DeepDiveReportFieldsV1),
    DynDiveV1(DynamicReportFieldsV1),
    StaticHoldsV1(StaticReportFieldsV1),
}

// #[derive(Serialize, Deserialize, SimpleObject, Clone)]
// pub struct UniqueApneaActivities {
//     pub activities: Vec<UniqueApneaActivity>,
// }

impl FromSql<sql_types::Jsonb, Pg> for UniqueApneaActivity {
    fn from_sql(value: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        let bytes = value.as_bytes();
        if bytes[0] != 1 {
            return Err("Unsupported JSONB encoding version".into());
        }
        serde_json::from_slice(&bytes[1..])
            .map_err(|_| "Invalid FormResponse Json: bad migration".into())
    }
}

impl UniqueApneaActivity {
    pub fn from_input(input: UniqueApneaActivityRequest) -> Self {
        match input {
            UniqueApneaActivityRequest::DeepDiveV1(deepv1) => {
                UniqueApneaActivity::DeepDiveV1(deepv1)
            }
            UniqueApneaActivityRequest::DynDiveV1(dynv1) => UniqueApneaActivity::DynDiveV1(dynv1),
            UniqueApneaActivityRequest::StaticHoldsV1(stav1) => {
                UniqueApneaActivity::StaticHoldsV1(stav1)
            }
        }
    }
}
