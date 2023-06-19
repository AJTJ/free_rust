use async_graphql::InputObject;

// AUTH STUFF
#[derive(InputObject, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct Logout {
    pub email: String,
}
