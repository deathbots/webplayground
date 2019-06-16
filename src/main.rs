#![feature(proc_macro_hygiene, decl_macro)]

use rocket::response::content;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> content::Html<String> {
    content::Html(include_str!("../ui/dist/index.html").to_string())
}

#[get("/bundle.js")]
fn bundle() -> content::Html<String> {
    content::Html(include_str!("../ui/dist/bundle.js").to_string())
}

fn main() {
    rocket::ignite().mount("/", routes![index, bundle]).launch();
}
