// use actix_web::{get, post, web, Error, HttpResponse, Responder};
use 
use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject, ID};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use slab::Slab;
use std::sync::Mutex;

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

// query {
//     getAllTestObjects {
//       val
//     }
//   }

#[Object]
impl QueryRoot {
    async fn get_all_test_objects(&self, ctx: &Context<'_>) -> Vec<TestObject> {
        let storage = ctx.data_unchecked::<Storage>().lock().unwrap();
        // let db = ctx.data_unchecked::<DbPool>();
        let vals = storage.iter().map(|(_, ob)| ob).cloned().collect();
        vals
    }

    async fn get_all_users(&self, ctx: &Context<'_>) -> u32 {
        let el = 
        34
    }
}

// mutation {
//     addTestObject(val: "memeggqqs")
//   }

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
}
