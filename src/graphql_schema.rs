use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject, ID};
use slab::Slab;
use std::sync::Mutex;

pub type DiveQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Debug, Clone, SimpleObject)]
pub struct TestObject {
    id: ID,
    val: String,
}

pub type Storage = Mutex<Slab<TestObject>>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_vals(&self, ctx: &Context<'_>) -> Vec<TestObject> {
        println!("query hit");
        let storage = ctx.data_unchecked::<Storage>().lock().unwrap();
        let vals = storage.iter().map(|(_, ob)| ob).cloned().collect();
        println!("{:?}", vals);
        vals
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn update_value(&self, ctx: &Context<'_>, val: String) -> ID {
        println!("mutation hit");
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
