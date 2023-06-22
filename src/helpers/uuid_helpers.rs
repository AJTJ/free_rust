use async_graphql::ID;
use uuid::Uuid;

use crate::errors::BigError;

pub fn async_id_to_uuid(id: &ID) -> Result<Uuid, BigError> {
    Uuid::parse_str(id).map_err(|e| BigError::UuidParsingerror { source: e })
}
