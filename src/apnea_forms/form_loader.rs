use super::actions::get_forms_by_id::get_forms_by_id;
use super::actions::get_forms_by_user::get_forms_by_user;
use super::dto::form_dto::Form;
use crate::graphql_schema::DbPool;
use crate::utility::errors::ActixBlockingSnafu;
use crate::utility::errors::BigError;
use actix_web::web;
use async_graphql::async_trait;
use async_graphql::dataloader::*;
use snafu::ResultExt;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct FormLoader(DbPool);

impl FormLoader {
    pub fn new(postgres_pool: DbPool) -> Self {
        Self(postgres_pool)
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for FormLoader {
    type Value = Form;
    type Error = Arc<BigError>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let pool = self.0.clone();
        let my_keys = keys.to_vec();
        let output = web::block(move || {
            let mut conn = pool.get().unwrap();
            get_forms_by_id(&mut conn, my_keys)
        })
        .await
        .context(ActixBlockingSnafu)??;

        let mut m: HashMap<Uuid, Form> = HashMap::new();
        if let Some(forms) = output {
            for form in forms.into_iter() {
                m.insert(form.id, form);
            }
        }
        Ok(m)
    }
}
