use crate::apnea_forms::resolvers::{Mutation as FormMutation, Query as FormQuery};
use crate::apnea_sessions::resolvers::{
    Mutation as ApneaSessionMutation, Query as ApneaSessionQuery,
};
use crate::auth::resolvers::{Mutation as AuthMutation, Query as AuthQuery};
use async_graphql::MergedObject;
use async_graphql::{EmptySubscription, Schema};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DiveQLSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct Query(FormQuery, ApneaSessionQuery, AuthQuery);
#[derive(MergedObject, Default)]
pub struct Mutation(FormMutation, ApneaSessionMutation, AuthMutation);

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
