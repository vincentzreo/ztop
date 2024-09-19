mod health_check;
mod subscriptions;
pub use health_check::*;
use serde::Deserialize;
pub use subscriptions::*;

#[allow(unused)]
#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
