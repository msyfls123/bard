use std::collections::BTreeMap;
use rocket_dyn_templates::Template;
use rocket::{Request, request::{FromRequest, Outcome}, http::Status};
use rocket_auth::{User};
use serde_json::{Value};

pub struct SPAInfo {
    user: Option<User>
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for SPAInfo {
    type Error = ();
    async fn from_request(req: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        match User::from_request(req).await {
            Outcome::Success(user) => Outcome::Success(SPAInfo {
                user: Some(user),
            }),
            Outcome::Error(_) => Outcome::Success(SPAInfo {
                user: None,
            }),
            Outcome::Forward(_) => Outcome::Error((Status::from_code(401).unwrap(), ())),
        }
        
    }

}

pub fn render_spa(spa_info: &SPAInfo) -> Template {
    let mut data: BTreeMap<String, Value> = BTreeMap::new();
    data.insert("name".to_string(), Value::from("kimi"));
    match &spa_info.user {
        Some(user) => {
            data.insert("user".to_string(), json!(user));
        },
        None => {
            data.insert("user".to_string(), Value::Null);
        }
    }
    Template::render("index", &data)
}