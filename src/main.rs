#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate serde;
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

use crate::inputs::DeclineInput;
use crate::inputs::DeclineParameters;
use crate::inputs::DeclineSegment;
use crate::inputs::Exponential;
use crate::inputs::ForecastParameter;

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

//Functions to extract data from incoming JSON

pub fn createExponential(input: Json<DeclineSegment>) -> Exponential<f32> {
    //Initializing the array
    let mut input_values: [ForecastParameter<f32>; 5] = [ForecastParameter::Unknown; 5];

    for (i, item) in input.parameters.iter().enumerate() {
        let val = match &item.input {
            None => ForecastParameter::Unknown,
            Some(x) => ForecastParameter::Known(*x),
        };

        input_values[i] = val;
    }

    let decline: Exponential<f32> = Exponential {
        qi: input_values[0],
        qf: input_values[1],
        d: input_values[2],
        duration: input_values[3],
        reserves: input_values[4],
    };

    return decline;
}

#[post("/solve", format = "json", data = "<data>")]
fn solve(data: Json<DeclineSegment>) -> Json<DeclineParameters> {
    println!("{:?}", data);

    //Create functions to parse incoming JSON

    //Create Knowns array

    let mut decline = createExponential(data);
    decline = decline.solve_unknowns();

    let decline_parameters = Json(decline.extract_parameters());

    println!("{:?}", decline_parameters.parameters);

    decline_parameters
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
