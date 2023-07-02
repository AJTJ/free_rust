use crate::apnea_forms::resolvers::{ApneaFormsMutation, ApneaFormsQuery};
use crate::apnea_sessions::resolvers::{ApneaSessionsMutation, ApneaSessionsQuery};
use crate::auth::resolvers::{AuthMutation, AuthQuery};
use async_graphql::MergedObject;
use async_graphql::{EmptySubscription, Schema};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DiveQLSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct Query(ApneaFormsQuery, ApneaSessionsQuery, AuthQuery);
#[derive(MergedObject, Default)]
pub struct Mutation(ApneaFormsMutation, ApneaSessionsMutation, AuthMutation);

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
