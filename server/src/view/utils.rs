use std::collections::BTreeMap;
use rocket_dyn_templates::Template;

pub fn render_spa() -> Template {
    let mut data = BTreeMap::new();
    data.insert("name".to_string(), "kimi".to_string());
    Template::render("index", &data)
}