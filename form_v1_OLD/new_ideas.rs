// use super::enums::DisciplinesEnum;
// use crate::{
//     apnea_forms::{
//         actions::{archive_form::archive_form, insert_form::insert_form},
//         dto::form_dto::{Form, FormDetails},
//         helpers::FormResponse,
//     },
//     utility::errors::BigError,
// };
// use async_graphql::{
//     Context, Enum, InputObject, Interface, Object, OneofObject, SimpleObject, Union,
// };
// use itertools::Itertools;
// use serde::{Deserialize, Serialize};
// // use snafu::ResultExt;
// // use strum::{Display, EnumIter, EnumString};
// use uuid::Uuid;
// /*
//   Using the other tools at my disposal with gql, what could I create?

//   [ ] A useful request type
//   [ ] A useful response type
//   [ ] Easy to transform a response type into a request type (in the client)
//   [ ] Easy to transform a request type into a response type (in the server)
//   A ResponseForm is ideally an array of fields.
//   If it's an indexed array, then we already have the order.

//   A RequestForm is ideally created as an array of fields as well.

// */
// #[derive(Serialize, Deserialize, Debug, SimpleObject, InputObject, Clone)]
// pub struct SessionNameV1 {
//     name: Option<String>,
//     // defaults
//     field_order: Option<i32>,
//     short_description: String,
// }

// // #[derive(Default, Serialize, Deserialize, SimpleObject, Debug, Clone)]
// // pub struct SessionNameV1 {
// //     name: Option<String>,
// //     // defaults
// //     field_order: Option<i32>,
// //     short_description: String,
// // }

// // #[derive(Serialize, Deserialize, Debug, InputObject, SimpleObject, Clone)]
// // pub struct WildlifeV1 {
// //     value: Option<WildlifeEnumV1>,
// //     // defaults
// //     field_order: Option<i32>,
// // }

// // #[derive(Serialize, Deserialize, Debug, SimpleObject, Clone)]
// // pub struct WildlifeV1 {
// //     value: Option<WildlifeEnumV1>,
// //     // defaults
// //     field_order: Option<i32>,
// //     short_description: String,
// // }

// #[derive(OneofObject, Serialize, Deserialize, Debug, Clone)]
// pub enum RequestFieldsV1 {
//     SessionNameV1(SessionNameV1),
//     // WildlifeV1(WildlifeV1),
// }

// pub type RequestFormFieldsV1 = Vec<RequestFieldsV1>;

// #[derive(Union, Serialize, Deserialize, Debug, Clone)]
// pub enum ResponseFieldsV1 {
//     SessionNameV1(SessionNameV1),
//     // WildlifeV1(WildlifeV1),
// }

// pub type ResponseFormFieldsV1 = Vec<ResponseFieldsV1>;

// // field(name = "scale", type = "Shape", arg(name = "s", type = "f32")),

// #[cfg(test)]
// mod tests {
//     use itertools::Itertools;

//     use super::{ResponseFieldsV1, ResponseFormFieldsV1};

//     #[test]
//     fn it_works() {
//         let some_fields: ResponseFormFieldsV1 = vec![
//             ResponseFieldsV1::SessionNameV1(super::SessionNameV1 {
//                 name: Some("memes".to_string()),
//                 field_order: Some(1),
//                 short_description: "hello".to_string(),
//             }),
//             ResponseFieldsV1::SessionNameV1(super::SessionNameV1 {
//                 name: Some("memes2".to_string()),
//                 field_order: Some(3),
//                 short_description: "hello".to_string(),
//             }),
//         ];

//         let sorted = some_fields
//             .into_iter()
//             .unique_by(|e| std::mem::discriminant(e))
//             .collect::<Vec<ResponseFieldsV1>>();
//     }
// }
