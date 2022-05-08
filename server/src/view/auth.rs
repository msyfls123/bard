use rocket::{get, post, form::Form, response::Redirect, Request};
use rocket_auth::{Error, Auth, Signup, Login, User};
use rocket_dyn_templates::Template;
use rocket::serde::json::Json;
use serde_json::value::Value;

use super::utils::{render_spa, SPAInfo};

#[post("/signup", data="<form>")]
pub async fn signup(form: Form<Signup>, auth: Auth<'_>) -> Result<&'static str, Error> {
    auth.signup(&form).await?;
    auth.login(&form.into()).await?;
    Ok("You signed up.")
}

#[post("/login", data="<form>")]
pub async fn login(form: rocket::serde::json::Json<Login>, auth: Auth<'_>) -> Result<Json<Value>, Error> {
    auth.login(&form).await?;
    let res = json!({
        "status": "success",
        "message": "You're logged in.",
    });
    Ok(Json(res))
}

#[post("/logout")]
pub fn logout(auth: Auth<'_>) {
    auth.logout();
}

#[get("/me", format="json")]
pub fn me_api(option: Option<User>) -> Json<Value> {
    if let Some(user) = option {
        let res = json!({
            "id": user.id(),
            "email": user.email(),
            "is_admin": user.is_admin,
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
pub fn me(spa_info: SPAInfo, option: Option<User>) -> MeResult {
    if let Some(_) = option {
        MeResult::Template(render_spa(&spa_info))
    } else {
        MeResult::Redirect(Redirect::temporary(uri!(login_page)))
    }
}

#[get("/login")]
pub fn login_page(spa_info: SPAInfo, option: Option<User>) -> MeResult {
    if let Some(_) = option {
        MeResult::Redirect(Redirect::temporary(uri!(me)))
    } else {
        MeResult::Template(render_spa(&spa_info))
    }
}
