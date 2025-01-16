use rocket::{get, launch, routes};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/custom-maps")]
fn custom_maps() -> &'static str {
    "This shows the custom maps"
}
#[get("/golden")]
fn golden() -> &'static str {
    "This shows information about the golden berries"
}
#[get("/speedrun")]
fn speedrun() -> &'static str {
    "This shows information about my speedrun PBs"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![custom_maps])
        .mount("/", routes![golden])
        .mount("/", routes![speedrun])
        .attach(Template::fairing())
}
