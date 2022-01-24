use rocket::fairing::Fairing;

use handlebars::{Handlebars, HelperDef, RenderContext, Helper, Context, JsonRender, HelperResult, Output, RenderError};

extern crate rocket_dyn_templates;

use rocket_dyn_templates::Template;

fn debug_helper (h: &Helper, _: &Handlebars, _: &Context, rc: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
  let param = h.param(0).unwrap();

  out.write("debug >>> ")?;
  out.write(format!("{:?}", param.value()).as_ref())?;
  Ok(())
}

pub fn get_template() -> impl Fairing {
  Template::custom(|engines| {
    engines.handlebars.register_helper("debug", Box::new(debug_helper));
  })
}
