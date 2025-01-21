use rocket::{fs::FileServer, get, launch, routes};
use rocket_dyn_templates::{context, Template};

mod parsing;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {content: "Content goes here"})
}

#[get("/custom-maps")]
fn custom_maps() -> Template {
    let (maps, total_time, total_deaths) = parsing::parse_custom_maps();
    let total_deaths_string = format!("Total deaths: {}", total_deaths);
    Template::render(
        "custom_maps",
        context! {
        total_time: total_time,
        total_deaths: total_deaths_string,
        map_table: maps
        },
    )
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
        .mount("/css", FileServer::from("css"))
        .attach(Template::fairing())
}
