use crate::apnea_forms::resolvers::{ApneaFormsMutation, ApneaFormsQuery};
use crate::apnea_sessions::resolvers::{ApneaSessionsMutation, ApneaSessionsQuery};
use crate::auth::resolvers::{AuthMutation, AuthQuery};
use async_graphql::MergedObject;
use async_graphql::{EmptySubscription, Schema};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

#[derive(MergedObject, Default)]
pub struct Query(ApneaFormsQuery, ApneaSessionsQuery, AuthQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(ApneaFormsMutation, ApneaSessionsMutation, AuthMutation);

// DbPool type
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

// My GQL Schema type
pub type DiveQLSchema = Schema<Query, Mutation, EmptySubscription>;
