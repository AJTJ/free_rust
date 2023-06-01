use crate::actions::add_user::add_user;
use crate::actions::get_user::get_user;
use crate::data::LoginData;
use crate::data::UserInputData;
use crate::data::UserQueryData;
use actix_web::error;
use async_graphql::FieldResult;
use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject, ID};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::RunQueryDsl;

pub type DiveQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
pub struct QueryRoot;
pub struct MutationRoot;

#[derive(Debug, Clone, SimpleObject)]
pub struct TestObject {
    id: ID,
    val: String,
}

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[Object]
impl QueryRoot {
    async fn get_all_users(&self, ctx: &Context<'_>) -> FieldResult<Vec<UserQueryData>> {
        let all_users = {
            let pool_ctx = ctx.data::<DbPool>().unwrap();
            let pool = pool_ctx.get().unwrap();
            use crate::schema::users::dsl::*;
            users
                .load::<UserQueryData>(&pool)
                .expect("loading all users")
        };
        Ok(all_users)
    }

    async fn get_user(
        &self,
        ctx: &Context<'_>,
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
            let mut pool = pool_ctx.get().unwrap();
            add_user(&mut pool, user_data)
        }
        .map_err(error::ErrorInternalServerError)
        .unwrap();

        Ok(user)
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
