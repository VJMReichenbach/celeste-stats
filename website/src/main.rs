use rocket::{get, launch, routes};

#[get("/")]
fn index() -> &'static str {
    "This is the index page"
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
}
