#![recursion_limit = "1024"]

#[macro_use]
extern crate serde;

#[macro_use]
extern crate lazy_static;

use anyhow::Result;
use manifest::{LocationInformation, construct_manifest};

mod manifest;
mod utils;

fn main() -> Result<()> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get_physical())
        .build_global()?;

    let locations_information = LocationInformation::new("cities.json", "trips.json")?;

    construct_manifest("../src/Manifest.elm", &locations_information)?;

    println!("World and Manifest builds complete.");
    Ok(())
}
