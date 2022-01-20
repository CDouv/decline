#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;

use std::io;
use std::path::{Path, PathBuf};

use rocket::http::Method;
use serde::Deserialize;

use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;

use rocket::response::NamedFile;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions, Error};

//Setting up CORS
fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000/",
        "http://172.22.112.1:3000/",
        "http://localhost:8000/",
        "http://0.0.0.0:8000/",
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),

        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

#[get("/")]

fn index() -> io::Result<NamedFile> {
    NamedFile::open("build/index.html")
}

#[derive(Debug, PartialEq, Deserialize)]
struct ExponentialInput {
    text: String,
    symbol: String,
    units: String,
    calculate: bool,
    input: Option<f32>,
}

#[post("/solve", format = "json", data = "<user_input>")]
fn solve(user_input: Json<ExponentialInput>) -> String {
    let s = format!("testing {:?}", user_input);
    println!("{:?}", s);
    s
}

#[get("/<file..>")]

fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("build/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, files, solve])
        .attach(make_cors())
}

fn main() {
    rocket().launch();
}
