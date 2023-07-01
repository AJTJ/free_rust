use crate::actions::add_dive;
use crate::actions::add_dive_session;
use crate::actions::add_form;
use crate::actions::get_completed_forms_by_user_id;
use crate::actions::get_dive_sessions_by_user;
use crate::actions::get_dives_by_user;
use crate::actions::get_form_fields_by_form;
use crate::actions::get_form_structures;
use crate::actions::get_forms_by_user_id;
use crate::actions::get_user_id_from_token_and_session;
use crate::actions::get_user_with_email;
use crate::actions::insert_completed_form;
use crate::actions::insert_user;
use crate::actions::login;
use crate::actions::logout;
use crate::actions::update_dive;
use crate::actions::update_dive_session;
use crate::apnea_forms::form_helper::FormStructureOutput;
use crate::apnea_forms::resolvers::{Mutation as FormMutation, Query as FormQuery};
use crate::apnea_sessions::resolvers::{
    Mutation as ApneaSessionMutation, Query as ApneaSessionQuery,
};
use crate::auth::resolvers::{Mutation as AuthMutation, Query as AuthQuery};
use crate::dto::auth_dto::Login;
use crate::dto::dive_dto::Dive;
use crate::dto::dive_dto::DiveFilter;
use crate::dto::dive_dto::DiveInput;
use crate::dto::dive_dto::DiveUpdate;
use crate::dto::dive_session_dto::DiveSession;
use crate::dto::dive_session_dto::DiveSessionFilter;
use crate::dto::dive_session_dto::DiveSessionInput;
use crate::dto::dive_session_dto::DiveSessionUpdate;
use crate::dto::form_dto::FormInput;
use crate::dto::form_dto::FormOutput;
use crate::dto::form_field_dto::FormField;
use crate::dto::query_dto::QueryParams;
use crate::dto::report_dto::CompletedFormInput;
use crate::dto::user_dto::{User, UserInput};
use crate::errors::{ActixBlockingSnafu, BigError};
use crate::graphql_query::gql_query;
use crate::guards::{DevelopmentGuard, LoggedInGuard};
use actix_web::web;
use async_graphql::MergedObject;
use async_graphql::{types::connection::*, Context, EmptySubscription, Object, Schema};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::RunQueryDsl;
use rand::prelude::*;
use snafu::ResultExt;
use uuid::Uuid;

pub type DiveQLSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(MergedObject, Default)]
pub struct Query(FormQuery, ApneaSessionQuery, AuthQuery);
#[derive(MergedObject, Default)]
pub struct Mutation(FormMutation, ApneaSessionMutation, AuthMutation);

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
