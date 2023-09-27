use async_graphql::ID as async_id;
use uuid::Uuid as extern_uuid;

// use crate::utility::errors::BigError;

// pub fn id_to_uuid(id: &async_id) -> Result<extern_uuid, BigError> {
//     extern_uuid::parse_str(id).map_err(|e| BigError::UuidParsingerror { source: e })
// }

// pub fn op_id_to_op_uuid(id: &Option<async_id>) -> Result<Option<extern_uuid>, BigError> {
//     id.clone().map(|id| id_to_uuid(&id)).transpose()
// }

// pub fn local_version_to_db_version(lv: &Vec<i32>) -> Vec<Option<i32>> {
//     lv.iter().map(|n| Some(*n)).collect()
// }

#[derive(Debug)]
pub struct MyId(async_id);
// pub struct Uuid(extern_uuid);
#[derive(Debug)]
pub struct MyUuid(extern_uuid);

impl From<MyId> for MyUuid {
    fn from(value: MyId) -> Self {
        let el = value.0 .0;
        MyUuid(extern_uuid::parse_str(&el).expect("bad conversion"))
    }
}

impl Into<MyId> for MyUuid {
    fn into(self) -> MyId {
        MyId(async_id::from(self.0))
    }
}

impl From<extern_uuid> for MyId {
    fn from(value: extern_uuid) -> Self {
        MyId(async_id::from(value))
    }
}
