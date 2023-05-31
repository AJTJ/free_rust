use crate::actions::add_user::add_user;
// use actix_web::{get, post, web, Error, HttpResponse, Responder};
use crate::schema::users::dsl::users;
use actix_web::error;
use actix_web::web;
use actix_web::Responder;
use async_graphql::FieldResult;
use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject, ID};
use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::RunQueryDsl;
// use diesel::Connection;
use slab::Slab;
use std::sync::Mutex;
use tracing::info;
use uuid::Uuid;

use crate::data::{UserCreationData, UserInputData};

pub type DiveQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
pub struct QueryRoot;
pub struct MutationRoot;

#[derive(Debug, Clone, SimpleObject)]
pub struct TestObject {
    id: ID,
    val: String,
}

pub type Storage = Mutex<Slab<TestObject>>;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[Object]
impl QueryRoot {
    async fn get_all_test_objects(&self, ctx: &Context<'_>) -> Vec<TestObject> {
        let storage = ctx.data_unchecked::<Storage>().lock().unwrap();
        // let db = ctx.data_unchecked::<DbPool>();
        let vals = storage.iter().map(|(_, ob)| ob).cloned().collect();
        vals
    }

    async fn get_all_users(&self, ctx: &Context<'_>) -> u32 {
        42
    }
}

#[Object]
impl MutationRoot {
    // async fn insert_test_object(&self, ctx: &Context<'_>, val: String) -> ID {
    //     let mut storage = ctx.data_unchecked::<Storage>().lock().unwrap();
    //     let entry = storage.vacant_entry();
    //     let id: ID = entry.key().into();
    //     let new_ob = TestObject {
    //         id: id.clone(),
    //         val,
    //     };
    //     storage.insert(new_ob);
    //     id
    // }

    async fn insert_user(
        &self,
        ctx: &Context<'_>,
        user_data: UserInputData,
    ) -> FieldResult<UserCreationData> {
        // NOTE: Should this be called in a "web::block" closure?
        // https://actix.rs/docs/databases/
        let user = {
            let pool_ctx = ctx.data::<DbPool>().unwrap();
            let mut pool = pool_ctx.get().unwrap();
            add_user(&mut pool, user_data)
        }
        .map_err(error::ErrorInternalServerError)
        .unwrap();

        Ok(user)
    }
}
