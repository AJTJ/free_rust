use async_graphql::ID;
use uuid::Uuid;

use crate::errors::BigError;

pub fn id_to_uuid(id: &ID) -> Result<Uuid, BigError> {
    Uuid::parse_str(id).map_err(|e| BigError::UuidParsingerror { source: e })
}

pub fn op_id_to_op_uuid(id: &Option<ID>) -> Result<Option<Uuid>, BigError> {
    id.map(|id| id_to_uuid(&id)).transpose()
}

pub fn local_version_to_db_version(lv: &Vec<i32>) -> Vec<Option<i32>> {
    lv.iter().map(|n| Some(*n)).collect()
}
