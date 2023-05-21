mod get;
mod post;

pub use get::*;
pub use post::*;

// pub struct User {
//     username: String,
//     password: String,
// }

// pub struct Client {
//     user: Option<User>,
//     token: Option<String>,
// }

// impl From<ClientBuilder> for Client {
//     fn from(value: ClientBuilder) -> Self {
//         Self { user: value.user, token: value.token }
//     }
// }

// pub struct ClientBuilder {
//     user: Option<User>,
//     token: Option<String>,
// }

// impl ClientBuilder {
//     pub fn new() -> Self {
//         Self { user: None, token: None }
//     }

//     pub async fn login(mut self) -> Self {
//         self
//     }

//     pub fn build(self) -> Client {
//         Client::from(self)
//     }
// }
