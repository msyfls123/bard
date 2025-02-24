use rocket_auth::User;
use rocket_dyn_templates::Template;

use super::utils::{render_spa, SPAInfo};

#[get("/container/<name>")]
pub fn container(name: String, spa_info: SPAInfo, option: Option<User>) -> Template {
    render_spa(&spa_info)
}
