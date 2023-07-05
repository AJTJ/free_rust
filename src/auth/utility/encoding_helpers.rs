use base64::{engine::general_purpose, Engine as _};
use snafu::ResultExt;

use crate::utility::errors::{BigError, DecodeSnafu};

use super::auth_data::UniversalIdType;
// use send_wrapper::SendWrapper;
// use std::ops::Deref;

pub fn get_encoded_id(id: UniversalIdType) -> String {
    general_purpose::STANDARD.encode(id)
}

pub fn decode_id(encoded_universal_id: String) -> Result<UniversalIdType, BigError> {
    let decoded = general_purpose::STANDARD
        .decode(encoded_universal_id)
        .context(DecodeSnafu)?;

    let decoded_id = decoded
        .as_slice()
        .try_into()
        .map_err(|e| BigError::TryFromSliceError { source: e });
    decoded_id
}
