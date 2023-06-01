use std::borrow::BorrowMut;
use std::time::Duration;

use crate::actions::add_user::add_user;
use crate::actions::get_user::get_user;
use crate::data::LoginData;
use crate::data::UserInputData;
use crate::data::UserQueryData;
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

pub struct LifetimeWrapper<'a>(&'a UserQueryData);

#[Object]
impl QueryRoot {
    async fn get_all_users<'ctx>(
        &self,
        inc_ctx: &Context<'ctx>,
    ) -> FieldResult<Vec<UserQueryData>> {
        let all_users = {
            let pool_ctx = inc_ctx.data_unchecked::<DbPool>();
            let pool = pool_ctx.get().unwrap();
            use crate::schema::users::dsl::*;
            users
                .load::<UserQueryData>(&pool)
                .expect("loading all users")
        };

        Ok(all_users)
    }

    async fn get_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        query_id: uuid::Uuid,
    ) -> FieldResult<UserQueryData> {
        let user = {
            let pool_ctx = ctx.data::<DbPool>().unwrap();
            let mut pool = pool_ctx.get().unwrap();
            get_user(&mut pool, query_id)
        }
        .map_err(error::ErrorInternalServerError)
        .unwrap();
        Ok(user)
    }

    // NOT WORKING
    async fn query_one<'a>(&self, ctx: &Context<'a>) -> FieldResult<i32> {
        web::block(move || {
            let pool_ctx = ctx.data::<DbPool>().unwrap();
            let conn = pool_ctx.get().unwrap();
            // do something with the conn here
        })
        .await?;

        Ok(42)
    }

    // async fn query_one(&self) -> FieldResult<i32> {
    //     info!("PRE QUERY ONE, thread: {:?}", std::thread::current().id());
    //     web::block(|| {
    //         std::thread::sleep(Duration::from_secs(5));
    //     })
    //     .await?;
    //     info!("POST QUERY ONE, thread: {:?}", std::thread::current().id());
    //     Ok(42)
    // }

    // async fn query_two(&self) -> FieldResult<i32> {
    //     info!("PRE QUERY two, thread: {:?}", std::thread::current().id());
    //     web::block(|| {
    //         std::thread::sleep(Duration::from_secs(5));
    //     })
    //     .await?;

    //     info!("POST QUERY two, thread: {:?}", std::thread::current().id());
    //     Ok(42)
    // }

    // DIVE SESSION

    // async fn get_session(&self) {
    //     unimplemented!()
    // }
}

#[Object]
impl MutationRoot {
    // USER THINGS
    async fn insert_user(
        &self,
        ctx: &Context<'_>,
        user_data: UserInputData,
    ) -> FieldResult<UserQueryData> {
        // TODO: Should this be called in a "web::block" closure?
        // https://actix.rs/docs/databases/

        let user = {
            let pool_ctx = ctx.data::<DbPool>().unwrap();
            let mut conn = pool_ctx.get().unwrap();
            add_user(&mut conn, user_data)
        }
        // .await
        .map_err(error::ErrorInternalServerError)
        .unwrap();

        Ok(user)
    }

    async fn delete_all_users(&self, ctx: &Context<'_>) -> FieldResult<usize> {
        let pool_ctx = ctx.data::<DbPool>().unwrap();
        let conn = pool_ctx.get().unwrap();
        use crate::schema::users::dsl::*;
        let deleted = diesel::delete(users).execute(&conn).unwrap();
        Ok(deleted)
    }

    // async fn login(&self, login_data: LoginData) -> FieldResult<UserQueryData> {
    //     /*
    //        login process
    //        - check email based on the email
    //        - do I need to trim whitespace?
    //        - is this good practice?
    //        -> return the user if success
    //     */
    //     unimplemented!()
    // }

    // async fn logout(&self) -> FieldResult<bool> {
    //     /*
    //        logout process
    //        - what information is needed upon logout?
    //        - confirmation
    //     */
    //     unimplemented!()
    // }

    // // DIVE SESSION
    // async fn add_session(&self) {}
    // async fn modify_session(&self) {}

    // // DIVES
    // async fn add_dive(&self) {}
    // async fn modify_dive(&self) {}
}

// async fn insert_user(
//     &self,
//     ctx: &Context<'_>,
//     user_data: UserInputData,
// ) -> FieldResult<UserQueryData> {
//     // TODO: Should this be called in a "web::block" closure?
//     // https://actix.rs/docs/databases/
//     let user = web::block(move || {
//         let pool_ctx = ctx.data::<DbPool>().unwrap();
//         let mut pool = pool_ctx.get().unwrap();
//         add_user(&mut pool, user_data)
//     })
//     .await?
//     .map_err(error::ErrorInternalServerError)
//     .unwrap();

//     Ok(user)
// }
