use rocket::{get, launch, routes};
use rocket_dyn_templates::{context, Template};

mod parsing;

#[get("/")]
fn index() -> Template {
    let file_content = parsing::parse_csv_file("custom_maps.csv");
    Template::render("index", context! {content: file_content})
}

#[get("/custom-maps")]
fn custom_maps() -> Template {
    Template::render("custom_maps", context! {})
}
#[get("/goldens")]
fn golden() -> Template {
    Template::render("goldens", context! {})
}
#[get("/speedruns")]
fn speedrun() -> Template {
    Template::render("speedruns", context! {})
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
