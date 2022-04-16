use rocket::{get, post, form::Form, response::Redirect};
use rocket_auth::{Error, Auth, Signup, Login, User};
use rocket_dyn_templates::Template;
use rocket::serde::json::Json;
use serde_json::value::Value;

use super::utils::render_spa;

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

#[get("/me", format="json")]
pub fn me_api(option: Option<User>) -> Json<Value> {
    if let Some(user) = option {
        let res = json!({
            "user": {
                "id": user.id(),
                "email": user.email(),
            }
        });
        Json(res)
    } else {
        let res = json!({});
        Json(res)
    }
}

#[derive(Debug, Responder)]
pub enum MeResult {
    Redirect(Redirect),
    Template(Template)
}

#[get("/me", rank=1)]
pub fn me(option: Option<User>) -> MeResult {
    if let Some(_) = option {
        MeResult::Template(render_spa())
    } else {
        MeResult::Redirect(Redirect::temporary(uri!(login_page)))
    }
}

#[get("/login")]
pub fn login_page(option: Option<User>) -> MeResult {
    if let Some(_) = option {
        MeResult::Redirect(Redirect::temporary(uri!(me)))
    } else {
        MeResult::Template(render_spa())
    }
}
