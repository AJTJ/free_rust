use async_graphql::{
    Context, EmptyMutation, EmptySubscription, Enum, Object, Result, Schema, SimpleObject,
    Subscription, ID,
};

use slab::Slab;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct QueryRoot;
pub struct MutationRoot;

pub type DiveQLSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(Clone, SimpleObject)]
pub struct TestObject {
    id: ID,
    name: String,
}

// #[Object]
// impl TestObject {
//     async fn get_id(&self) -> &str {
//         &self.id
//     }
// }

pub type Storage = Mutex<Slab<TestObject>>;

#[Object]
impl QueryRoot {
    async fn query_hello(&self, ctx: &Context<'_>, sent_int: u32) -> u32 {
        let mut storage = ctx.data_unchecked::<Storage>().lock();
        let all_obs = storage
            .iter()
            .map(|(_, test_object)| test_object)
            .cloned()
            .collect();
        println!("all obs: {}", all_obs);
        println!("query hit");
        sent_int
    }
}

#[Object]
impl MutationRoot {
    async fn update_value(&self, ctx: &Context<'_>, sent_num: u32) -> u32 {
        println!("mutation hit");
        sent_num
    }
}
