use crate::actions::add_user;
use crate::actions::get_user;
use crate::actions::login;
use crate::data::LoginData;
use crate::data::UserInputData;
use crate::data::UserQueryData;
use crate::Shared;
use actix_session::Session;
use actix_web::error;
use actix_web::web;
use async_graphql::FieldResult;
use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject, ID};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::RunQueryDsl;
use tracing::info;

pub type DiveQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
pub struct QueryRoot;
pub struct MutationRoot;

#[derive(Debug, Clone, SimpleObject)]
pub struct TestObject {
    id: ID,
    val: String,
}

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct Identity {
    pub id: Option<String>,
}

#[Object]
impl QueryRoot {
    async fn get_all_users<'ctx>(
        &self,
        inc_ctx: &Context<'ctx>,
    ) -> FieldResult<Vec<UserQueryData>> {
        let pool_ctx = inc_ctx.data_unchecked::<DbPool>().clone();

        let all_users = web::block(move || {
            let pool = pool_ctx.get().unwrap();
            use crate::schema::users::dsl::*;
            users
                .load::<UserQueryData>(&pool)
                .expect("loading all users")
        })
        .await?;

        Ok(all_users)
    }

    async fn get_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        query_id: uuid::Uuid,
    ) -> FieldResult<UserQueryData> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let user = web::block(move || {
            let mut pool = pool_ctx.get().unwrap();
            get_user(&mut pool, query_id)
        })
        .await?
        .map_err(error::ErrorInternalServerError)
        .unwrap();
        Ok(user)
    }

    // DIVE SESSION

    // async fn get_session(&self) {
    //     unimplemented!()
    // }
}

#[Object]
impl MutationRoot {
    async fn insert_user(
        &self,
        ctx: &Context<'_>,
        user_data: UserInputData,
    ) -> FieldResult<UserQueryData> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let user = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            add_user(&mut conn, user_data)
        })
        .await?
        .map_err(error::ErrorInternalServerError)
        .unwrap();

        Ok(user)
    }

    async fn delete_all_users(&self, ctx: &Context<'_>) -> FieldResult<usize> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let deleted = web::block(move || {
            let conn = pool_ctx.get().unwrap();
            use crate::schema::users::dsl::*;
            diesel::delete(users).execute(&conn).unwrap()
        })
        .await?;

        Ok(deleted)
    }

    async fn login(&self, ctx: &Context<'_>, login_data: LoginData) -> FieldResult<UserQueryData> {
        let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
        let maybe_user = web::block(move || {
            let mut conn = pool_ctx.get().unwrap();
            login(&mut conn, login_data.email, login_data.hashed_password)
        })
        .await
        .expect("login web:block error")
        .expect("problem getting login user");

        // return the user if found
        match maybe_user

        // TODO: If user/pw isn't found, then need better server response
    }

    // async fn login(&self, ctx: &Context<'_>, login_data: LoginData) -> FieldResult<UserQueryData> {
    //     let pool_ctx = ctx.data_unchecked::<DbPool>().clone();
    //     let id_ctx = ctx.data_unchecked::<Identity>().clone();
    //     info!("the id: {:?}", id_ctx.id);

    //     // check if the email/pw combo finds a user
    //     let user = web::block(move || {
    //         let mut pool = pool_ctx.get().unwrap();
    //         login(&mut pool, login_data.email, login_data.hashed_password)
    //     })
    //     .await?
    //     .map_err(error::ErrorInternalServerError)
    //     .unwrap();

    //     // return the user if found
    //     Ok(user)

    //     // TODO: If user/pw isn't found, then need better server response
    // }

    // async fn logout(&self, req: HttpRequest) -> FieldResult<i32> {
    //     // user.logout();
    //     Ok(42)
    // }

    // // DIVE SESSION
    // async fn add_session(&self) {}
    // async fn modify_session(&self) {}

    // // DIVES
    // async fn add_dive(&self) {}
    // async fn modify_dive(&self) {}
}
