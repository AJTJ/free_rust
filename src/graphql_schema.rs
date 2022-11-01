// use actix_web::{get, post, web, Error, HttpResponse, Responder};
use crate::schema::users::dsl::users;
use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject, ID};
use chrono::{naive, NaiveDateTime, Utc};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Error, Pool};
// use diesel::Connection;
use slab::Slab;
use std::sync::Mutex;
// use uuid::Uuid;

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

type DbPool = Pool<ConnectionManager<PgConnection>>;

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
    async fn add_test_object(&self, ctx: &Context<'_>, val: String) -> ID {
        let mut storage = ctx.data_unchecked::<Storage>().lock().unwrap();
        let entry = storage.vacant_entry();
        let id: ID = entry.key().into();
        let new_ob = TestObject {
            id: id.clone(),
            val,
        };
        storage.insert(new_ob);
        id
    }

    async fn add_user(&self, ctx: &Context<'_>, user_data: UserInputData) -> u32 {
        let db = ctx.data_unchecked::<DbPool>();

        let pool = db.get().unwrap();

        let current_stamp = Utc::now().naive_utc();

        let new_user = UserCreationData {
            username: user_data.username,
            hashed_password: user_data.hashed_password,
            email: user_data.email,
            created_at: current_stamp,
            updated_at: current_stamp,
        };

        let el = pool.build_transaction().run(|| {});

        let inserted = diesel::insert_into(users).values(new_user);

        42
    }
}
