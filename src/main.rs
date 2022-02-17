#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate serde;
pub mod exponential;
pub mod hyperbolic;
pub mod inputs;

use std::io;
use std::path::{Path, PathBuf};

use rocket::http::Method;
use rocket::Data;

use serde::Deserialize;
use serde_json::Result as JsonResult;
use serde_json::{Deserializer, Value};

use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;

use rocket::response::NamedFile;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions, Error};

use crate::exponential::Exponential;
use crate::hyperbolic::Hyperbolic;
use crate::inputs::convert_inputs;
use crate::inputs::createExponential;
use crate::inputs::createHyperbolic;
use crate::inputs::DeclineParameters;
use crate::inputs::DeclineSegment;
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

#[post("/solve", format = "json", data = "<data>")]
fn solve(data: Json<Vec<DeclineSegment>>) -> Json<Vec<DeclineParameters>> {
    println!("{:?}", data);

    //instantiate Vec of values
    let mut decline_segments: Vec<DeclineParameters> = vec![];

    //Create functions to parse incoming JSON

    for segment in data.iter() {
        let forecast_type: &str = &*segment.forecastType;

        if (forecast_type == "exponential") {
            let mut decline: Exponential<f32> = createExponential(segment);
            decline = decline.solve_unknowns();
            let decline_parameters = decline.extract_parameters();
            decline_segments.push(decline_parameters);
        } else if (forecast_type == "hyperbolic") {
            let mut decline: Hyperbolic<f32> = createHyperbolic(segment);
            decline = decline.solve_unknowns();
            let decline_parameters = decline.extract_parameters();
            decline_segments.push(decline_parameters);
        } else {
            panic!();
        }
    }

    Json(decline_segments)
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
