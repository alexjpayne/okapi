use rocket_okapi::openapi;
use rocket_okapi::swagger_ui::{SwaggerUIConfig, make_swagger_ui};

#[macro_use]
extern crate rocket;

#[openapi]
#[get("/hello/<type>")]
fn hello(r#type: String) ->  String {
  r#type
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/openapi.json".to_string(),
        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/swagger", make_swagger_ui(&get_docs()))
}
