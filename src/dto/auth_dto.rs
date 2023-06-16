use async_graphql::InputObject;

// AUTH STUFF
#[derive(InputObject, Debug)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct LogoutData {
    pub email: String,
}
