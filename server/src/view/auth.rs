use rocket::{get, post, form::Form};
use rocket_auth::{Error, Auth, Signup, Login};
use std::collections::BTreeMap;
use rocket_dyn_templates::Template;

#[post("/signup", data="<form>")]
pub async fn signup(form: Form<Signup>, auth: Auth<'_>) -> Result<&'static str, Error> {
    auth.signup(&form).await?;
    auth.login(&form.into()).await?;
    Ok("You signed up.")
}

#[post("/login", data="<form>")]
pub async fn login(form: rocket::serde::json::Json<Login>, auth: Auth<'_>) -> Result<&'static str, Error> {
    auth.login(&form).await?;
    Ok("You're logged in.")
}

#[post("/logout")]
pub fn logout(auth: Auth<'_>) {
    auth.logout();
}

#[get("/me")]
pub async fn me(auth: Auth<'_>) -> String {
    format!("{:?}", auth.get_user().await)
}

#[get("/login")]
pub fn login_page() -> Template {
    let mut data = BTreeMap::new();
    data.insert("name".to_string(), "kimi".to_string());
    Template::render("index", &data)
}