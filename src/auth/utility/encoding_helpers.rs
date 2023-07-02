use base64::{engine::general_purpose, Engine as _};

use super::auth_data::UniversalIdType;
// use send_wrapper::SendWrapper;
// use std::ops::Deref;

pub fn get_encoded_id(id: UniversalIdType) -> String {
    general_purpose::STANDARD.encode(id)
}

pub fn decode_id(encoded_universal_id: String) -> UniversalIdType {
    let decoded = general_purpose::STANDARD
        .decode(encoded_universal_id)
        .expect("error decoding id");

    let decoded_id: UniversalIdType = decoded
        .as_slice()
        .try_into()
        .expect("doesn't somehow fit into my type");
    decoded_id
}

// UNUSED
// #[derive(Clone, Debug)]
// pub struct Shared<T>(pub Option<SendWrapper<T>>);

// impl<T> Shared<T> {
//     pub fn new(v: T) -> Self {
//         Self(Some(SendWrapper::new(v)))
//     }
// }

// impl<T> Deref for Shared<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &*self.0.as_deref().clone().unwrap()
//     }
// }
