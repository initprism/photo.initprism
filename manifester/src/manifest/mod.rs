use anyhow::{anyhow, Error, Result};

use globwalk::DirEntry;
use image::imageops::FilterType::Lanczos3;
use image::GenericImageView;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::BTreeMap;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::process::Command;
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use url::Url;

use super::utils::*;

use map::*;
mod map;

use nominatim::Nominatim;
mod nominatim;

const CCA3_JSON: &str = include_str!("..\\world\\cca3.json");
const MANIFEST_YAML: &str = include_str!("..\\world\\manifest.yaml");
const ATTRIBUTION_YAML: &str = include_str!("..\\world\\attribution.yaml");

lazy_static! {
    static ref CCA3: CountryCode = serde_json::from_str(&CCA3_JSON).unwrap();
    static ref MANIFEST: Manifest = serde_yaml::from_str(&MANIFEST_YAML).unwrap();
    static ref ATTRIBUTION: Attribution = serde_yaml::from_str(&ATTRIBUTION_YAML).unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct Manifest {
    places: BTreeMap<Country, BTreeMap<Location, Option<String>>>,
    trips: Vec<Trip>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CountryCode {
    #[serde(with = "codes")]
    codes: BTreeMap<String, String>,
}

mod codes {
    use std::collections::BTreeMap;

    use serde::de::{Deserialize, Deserializer};
    use serde::ser::Serializer;

    #[derive(Debug, Serialize, Deserialize)]
    struct CountryCodeStruct {
        name: String,
        #[serde(rename = "alpha-3")]
        alpha3: String,
    }

    pub fn serialize<S>(map: &BTreeMap<String, String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(map.values())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BTreeMap<String, String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = BTreeMap::new();
        for item in Vec::<CountryCodeStruct>::deserialize(deserializer)? {
            map.insert(item.name, item.alpha3);
        }
        Ok(map)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Attribution {
    marked: bool,
    usage_terms: String,
    web_statement: Url,
    license: Url,
    more_permissions: Url,
    attribution_url: Url,
    attribution_name: String,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct LocationInformation {
    id: Location,
    name: String,
    country: Country,
    coordinates: Vec<f32>,
}

impl LocationInformation {
    pub fn new<P1, P2>(out_cities: P1, out_trips: P2) -> Result<Vec<Self>>
    where
        P1: AsRef<std::path::Path>,
        P2: AsRef<std::path::Path>,
    {
        let pause = Duration::from_secs(1);

        let cities_buffer = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(&out_cities);

        let mut locations_details: Vec<LocationInformation> = Vec::new();

        match cities_buffer {
            Ok(buffer) => {
                // Add new info to cities.json
                let mut cities: FeatureCollection = serde_json::from_reader(&buffer)?;

                let (new_countries, new_locations) = get_new_places(&cities);

                for (country, locations) in &MANIFEST.places {
                    for (location, local_name) in
                        locations.iter().filter(|(l, _)| **l != Location::Local)
                    {
                        let coordinates = if new_locations.contains(location) {
                            let coords = Nominatim::search(&format!(
                                "{}, {}",
                                location.name(),
                                country.name()
                            ))?;

                            thread::sleep(pause);

                            let properties = Properties {
                                name: location.name(),
                                localname: local_name.to_owned(),
                                country: Some(country.code(&CCA3.codes)?),
                            };

                            let coordinates =
                                vec![coords.lon().parse::<f32>()?, coords.lat().parse::<f32>()?];

                            let geometry = Geometry {
                                type_: "Point".to_string(),
                                coordinates: Coordinates::Point(coordinates.clone()),
                            };

                            cities.features.push(Feature {
                                type_: "Feature".to_string(),
                                properties,
                                geometry,
                            });

                            coordinates
                        } else {
                            location.feature_coordinates(&cities.features)?
                        };

                        locations_details.push(LocationInformation {
                            id: location.clone(),
                            name: location.name(),
                            country: country.clone(),
                            coordinates,
                        });
                    }
                }

                if new_countries.len() + new_locations.len() > 0 {
                    let cities_writer = OpenOptions::new()
                        .write(true)
                        .truncate(true) //We must truncate the file before writing the new data.
                        .open(&out_cities)?;
                    serde_json::to_writer(&cities_writer, &cities)?;
                }

                write_trip(&cities.features, out_trips)?;
            }
            Err(_) => {
                let mut features: Vec<Feature> = Vec::new();

                for (country, locations) in &MANIFEST.places {
                    for (location, local_name) in
                        locations.iter().filter(|(l, _)| **l != Location::Local)
                    {
                        let coords =
                            Nominatim::search(&format!("{}, {}", location.name(), country.name()))?;
                        thread::sleep(pause);

                        let properties = Properties {
                            name: location.name(),
                            localname: local_name.to_owned(),
                            country: Some(country.code(&CCA3.codes)?),
                        };
                        let coordinates =
                            vec![coords.lon().parse::<f32>()?, coords.lat().parse::<f32>()?];
                        let geometry = Geometry {
                            type_: "Point".to_string(),
                            coordinates: Coordinates::Point(coordinates.clone()),
                        };

                        println!("{} {:?}", location.to_string(), geometry.coordinates);
                        features.push(Feature {
                            type_: "Feature".to_string(),
                            properties,
                            geometry,
                        });
                        locations_details.push(LocationInformation {
                            id: location.clone(),
                            name: location.name(),
                            country: country.clone(),
                            coordinates,
                        });
                    }
                }

                write_trip(&features, out_trips)?;

                let cities = FeatureCollection {
                    type_: "FeatureCollection".to_string(),
                    features,
                };

                let cities_create = File::create(&out_cities)?;
                serde_json::to_writer(&cities_create, &cities)?;
            }
        };

        Ok(locations_details)
    }
}

fn get_new_places(cities: &FeatureCollection) -> (Vec<Country>, Vec<Location>) {
    let cities_country_codes = cities
        .features
        .iter()
        .map(|f| f.properties.country.clone().unwrap())
        .collect::<Vec<String>>();

    let mut cities_countries: Vec<String> = Vec::new();

    for c in cities_country_codes {
        for (name, code) in &CCA3.codes {
            if c == *code {
                let mut identifier = name.clone().to_string();
                identifier.retain(|c| c != ' ');
                cities_countries.push(identifier);
                break;
            }
        }
    }

    let config_countries = MANIFEST
        .places
        .iter()
        .map(|(name, _)| name)
        .cloned()
        .collect::<Vec<Country>>();

    let mut new_countries: Vec<Country> = Vec::new();
    for ctry in config_countries {
        if !cities_countries.contains(&ctry.to_string()) {
            new_countries.push(ctry);
        }
    }

    let cities_locations = cities
        .features
        .iter()
        .map(|f| to_location_identfier_string(&f.properties.name))
        .collect::<Vec<String>>();

    let config_locations = MANIFEST
        .places
        .iter()
        .map(|(_, locations)| {
            locations
                .iter()
                .map(|(name, _)| name)
                .cloned()
                .collect::<Vec<Location>>()
        })
        .flatten()
        .collect::<Vec<Location>>();

    let mut new_locations: Vec<Location> = Vec::new();

    for loc in config_locations {
        if !cities_locations.contains(&loc.to_string()) && loc != Location::Local {
            new_locations.push(loc);
        }
    }
    (new_countries, new_locations)
}

fn write_trip<P>(features: &[Feature], trips_json: P) -> Result<()>
where
    P: AsRef<std::path::Path>,
{
    let mut trip_features: Vec<Feature> = Vec::new();

    for trip in &MANIFEST.trips {
        let properties = Properties {
            name: trip.name.clone(),
            localname: None,
            country: None,
        };

        let mut coords: Vec<Vec<f32>> = Vec::new();
        for city in &trip.cities {
            coords.push(city.feature_coordinates(&features)?);
        }

        let geometry = Geometry {
            type_: "LineString".to_string(),
            coordinates: Coordinates::LineString(coords),
        };

        trip_features.push(Feature {
            type_: "Feature".to_string(),
            properties,
            geometry,
        });
    }

    let trips = FeatureCollection {
        type_: "FeatureCollection".to_string(),
        features: trip_features,
    };
    let trips_buffer = File::create(trips_json)?;
    serde_json::to_writer(&trips_buffer, &trips)?;

    Ok(())
}

pub fn construct_manifest<P>(
    out_elm: P,
    locations_information: &[LocationInformation],
) -> Result<()>
where
    P: AsRef<std::path::Path>,
{
    println!("Building Manifest.");

    let mut manifest = File::create(&out_elm)?;

    writeln!(manifest, "module Manifest exposing (Country(..), Date, Image, Location(..), Month(..), Trip(..), Year, countryId, countryList, countryLocalName, countryName, locationInformation, locationList, locationLocalName, manifest, stringToCountry, stringToLocation, stringToTrip, tripInformation, tripList)")?;

    writeln!(manifest, "-- COUNTRIES")?;
    write_countries(&mut manifest)?;

    writeln!(manifest, "-- LOCATIONS")?;
    write_locations(&mut manifest, locations_information)?;

    writeln!(manifest, "-- TRIPS")?;
    write_trips(&mut manifest)?;

    writeln!(manifest, "-- MANIFEST")?;
    write_manifest(&mut manifest)?;

    Command::new("cmd")
        .args(&["/c", "elm-format"])
        .arg("--elm-version=0.19")
        .arg("--yes")
        .arg("../src/Manifest.elm")
        .status()?;
    Ok(())
}

fn write_countries(manifest: &mut File) -> Result<()> {
    writeln!(manifest, "type Country")?;

    let mut idx = 0;
    for (cntry, _) in &MANIFEST.places {
        if idx != 0 {
            writeln!(manifest, "    | {}", cntry)?;
        } else {
            writeln!(manifest, "    = {}", cntry)?;
        }
        idx += 1;
    }

    writeln!(manifest, "countryList : List Country")?;
    writeln!(manifest, "countryList =")?;
    idx = 0;
    for (cntry, _) in &MANIFEST.places {
        if idx != 0 {
            writeln!(manifest, "    , {}", cntry)?;
        } else {
            writeln!(manifest, "    [ {}", cntry)?;
        }
        idx += 1;
    }
    writeln!(manifest, "    ]")?;

    writeln!(manifest, "countryId : Country -> String")?;
    writeln!(manifest, "countryId country =")?;
    writeln!(manifest, "    case country of")?;
    for (cntry, _) in &MANIFEST.places {
        writeln!(manifest, "        {} ->", cntry)?;
        writeln!(manifest, "            \"{}\"", cntry.code(&CCA3.codes)?)?;
    }

    writeln!(manifest, "countryName : Country -> String")?;
    writeln!(manifest, "countryName country =")?;
    writeln!(manifest, "    case country of")?;
    for (cntry, _) in &MANIFEST.places {
        writeln!(manifest, "        {} ->", cntry)?;
        writeln!(manifest, "            \"{}\"", cntry.name())?;
    }

    writeln!(manifest, "stringToCountry : String -> Maybe Country")?;
    writeln!(manifest, "stringToCountry country =")?;
    writeln!(manifest, "    case country of")?;
    for (cntry, _) in &MANIFEST.places {
        writeln!(manifest, "        \"{}\" ->", cntry.name())?;
        writeln!(manifest, "            Just {}", cntry)?;
    }
    writeln!(manifest, "        _ ->")?;
    writeln!(manifest, "            Nothing")?;

    writeln!(manifest, "countryLocalName : Country -> Maybe String")?;
    writeln!(manifest, "countryLocalName country =")?;
    writeln!(manifest, "    case country of")?;
    for (cntry, locations) in &MANIFEST.places {
        if let Some(Some(local)) = locations.get(&Location::Local) {
            writeln!(manifest, "        {} ->", cntry)?;
            writeln!(manifest, "            Just \"{}\"", local)?;
        };
    }
    // writeln!(manifest, "        _ ->")?;
    // writeln!(manifest, "            Nothing")?;

    Ok(())
}

fn write_locations(
    manifest: &mut File,
    locations_information: &[LocationInformation],
) -> Result<()> {
    let mut config_locations = MANIFEST
        .places
        .iter()
        .map(|(_, locations)| {
            locations
                .iter()
                .map(|(name, loc)| (name.clone(), loc.clone()))
                .collect::<Vec<(Location, Option<String>)>>()
        })
        .flatten()
        .filter(|(l, _)| *l != Location::Local)
        .collect::<Vec<(Location, Option<String>)>>();

    config_locations.sort();
    writeln!(manifest, "type Location")?;
    let mut idx = 0;
    for (loc, _) in &config_locations {
        if idx != 0 {
            writeln!(manifest, "    | {}", loc)?;
        } else {
            writeln!(manifest, "    = {}", loc)?;
        }
        idx += 1;
    }

    writeln!(manifest, "locationList : List Location")?;
    writeln!(manifest, "locationList =")?;
    idx = 0;
    for (loc, _) in &config_locations {
        if idx != 0 {
            writeln!(manifest, "    , {}", loc)?;
        } else {
            writeln!(manifest, "    [ {}", loc)?;
        }
        idx += 1;
    }
    writeln!(manifest, "    ]")?;

    writeln!(manifest, "stringToLocation : String -> Maybe Location")?;
    writeln!(manifest, "stringToLocation location =")?;
    writeln!(manifest, "    case location of")?;
    for (loc, _) in &config_locations {
        writeln!(manifest, "        \"{}\" ->", loc.name())?;
        writeln!(manifest, "            Just {}", loc)?;
    }
    writeln!(manifest, "        _ ->")?;
    writeln!(manifest, "            Nothing")?;

    writeln!(manifest, "locationLocalName : Location -> Maybe String")?;
    writeln!(manifest, "locationLocalName location =")?;
    writeln!(manifest, "    case location of")?;
    for (loc, local_name) in &config_locations {
        if let Some(local) = local_name {
            writeln!(manifest, "        {} ->", loc)?;
            writeln!(manifest, "            Just \"{}\"", local)?;
        };
    }
    // writeln!(manifest, "        _ ->")?;
    // writeln!(manifest, "            Nothing")?;

    writeln!(manifest, "type alias LocationInformation =")?;
    writeln!(manifest, "    {{ name : String")?;
    writeln!(manifest, "    , country : Country")?;
    writeln!(manifest, "    , coordinates : ( Float, Float )")?;
    writeln!(manifest, "    }}")?;

    writeln!(
        manifest,
        "locationInformation : Location -> LocationInformation"
    )?;
    writeln!(manifest, "locationInformation location =")?;
    writeln!(manifest, "    case location of")?;
    for info in locations_information {
        let lon = info
            .coordinates
            .get(0)
            .ok_or(anyhow!("No longitude value in coordinates"))?;

        let lat = info
            .coordinates
            .get(1)
            .ok_or(anyhow!("No latitude value in coordinates"))?;

        writeln!(manifest, "    {} ->", info.id)?;
        writeln!(manifest, "    {{ name = \"{}\"", info.name)?;
        writeln!(manifest, "    , country = {}", info.country)?;
        writeln!(manifest, "    , coordinates = ( {:.3}, {:.3} )", lon, lat)?;
        writeln!(manifest, "    }}")?;
    }
    Ok(())
}

fn write_trips(manifest: &mut File) -> Result<()> {
    writeln!(manifest, "type Trip")?;
    let mut idx = 0;

    for trip in &MANIFEST.trips {
        if idx != 0 {
            writeln!(manifest, "    | {}", trip.id_string())?;
        } else {
            writeln!(manifest, "    = {}", trip.id_string())?;
        }
        idx += 1;
    }

    writeln!(manifest, "tripList : List Trip")?;
    writeln!(manifest, "tripList =")?;
    idx = 0;
    for trip in &MANIFEST.trips {
        if idx != 0 {
            writeln!(manifest, "    , {}", trip.id_string())?;
        } else {
            writeln!(manifest, "    [ {}", trip.id_string())?;
        }
        idx += 1;
    }
    writeln!(manifest, "    ]")?;

    writeln!(manifest, "stringToTrip : String -> Maybe Trip")?;
    writeln!(manifest, "stringToTrip trip =")?;
    writeln!(manifest, "    case trip of")?;
    for trip in &MANIFEST.trips {
        writeln!(manifest, "        \"{}\" ->", trip.description)?;
        writeln!(manifest, "            Just {}", trip.id_string())?;
    }
    writeln!(manifest, "        _ ->")?;
    writeln!(manifest, "            Nothing")?;

    writeln!(manifest, "type alias TripInformation =")?;
    writeln!(manifest, "    {{ name : String")?;
    writeln!(manifest, "    , description : String")?;
    writeln!(manifest, "    , locations : List Location")?;
    writeln!(manifest, "    , dates : List Date")?;
    writeln!(manifest, "    }}")?;

    writeln!(manifest, "tripInformation : Trip -> TripInformation")?;
    writeln!(manifest, "tripInformation trip =")?;
    writeln!(manifest, "    case trip of")?;
    for trip in &MANIFEST.trips {
        writeln!(manifest, "        {} ->", trip.id_string())?;
        writeln!(manifest, "            {{ name = \"{}\"", trip.name)?;
        writeln!(
            manifest,
            "            , description = \"{}\"",
            trip.description
        )?;
        write!(manifest, "            , locations = [ ")?;
        idx = 0;
        for place in &trip.cities {
            if idx != 0 {
                write!(manifest, ", {}", place)?;
            } else {
                write!(manifest, "{}", place)?;
            }
            idx += 1;
        }
        writeln!(manifest, " ]")?;
        write!(manifest, "            , dates = [ ")?;
        idx = 0;
        for date in &trip.dates {
            let splitidx = date
                .find('/')
                .ok_or(anyhow!("{} has a malformed date string", trip.id_string()))?;
            let (year, month_str) = date.split_at(splitidx);
            let mut month_string = month_str.to_string();
            month_string.retain(|c| c != '/');
            if idx != 0 {
                write!(
                    manifest,
                    ", Date {} {}",
                    year,
                    Month::from_str(&month_string)?
                )?;
            } else {
                write!(
                    manifest,
                    "Date {} {}",
                    year,
                    Month::from_str(&month_string)?
                )?;
            }
            idx += 1;
        }
        writeln!(manifest, " ]")?;
        writeln!(manifest, "            }}")?;
    }

    // Extras, just to keep Date contained.
    writeln!(manifest, "type alias Year =")?;
    writeln!(manifest, "    Int")?;

    writeln!(manifest, "type Month")?;
    // No point in making Month an iterator.
    writeln!(manifest, "    = Jan")?;
    writeln!(manifest, "    | Feb")?;
    writeln!(manifest, "    | Mar")?;
    writeln!(manifest, "    | Apr")?;
    writeln!(manifest, "    | May")?;
    writeln!(manifest, "    | Jun")?;
    writeln!(manifest, "    | Jul")?;
    writeln!(manifest, "    | Aug")?;
    writeln!(manifest, "    | Sep")?;
    writeln!(manifest, "    | Oct")?;
    writeln!(manifest, "    | Nov")?;
    writeln!(manifest, "    | Dec")?;
    writeln!(manifest, "type alias Date =")?;
    writeln!(manifest, "    {{ year : Year")?;
    writeln!(manifest, "    , month : Month")?;
    writeln!(manifest, "    }}")?;

    Ok(())
}

fn write_manifest(manifest: &mut File) -> Result<()> {
    // Ignore the thumbnails and blurs at this point. We will check for them later.
    let walker = globwalk::GlobWalkerBuilder::from_patterns(
        "../dist/gallery/",
        &["*.{png,jpg,jpeg,PNG,JPG,JPEG}", "!*_small*", "!*_blur*"],
    )

    .follow_links(true)
    .build()?
    .into_iter()
    .filter_map(Result::ok)
    .collect::<Vec<DirEntry>>();

    let progcount = walker.len() as u64;
    let bar = ProgressBar::new(progcount);

    bar.set_style(ProgressStyle::default_bar().template("[{elapsed_precise}] {bar:25.cyan/blue} {pos:>5}/{len:5} {msg}")?);

    writeln!(manifest, "type alias Image =")?;
    writeln!(manifest, "    {{ file : String")?;
    writeln!(manifest, "    , date : Date")?;
    writeln!(manifest, "    , location : Location")?;
    writeln!(manifest, "    , aspectRatio : Float")?;
    writeln!(manifest, "    , description : String")?;
    writeln!(manifest, "    }}")?;

    writeln!(manifest, "manifest : List Image")?;
    writeln!(manifest, "manifest =")?;

    for (idx, file) in bar.wrap_iter(walker.iter().enumerate()) {
        let bar_msg = file
            .path()
            .strip_prefix("../dist/gallery/")?
            .to_str()
            .unwrap_or_default();

        if bar_msg.len() > 50 {
            let msg = bar_msg.split('/').collect::<Vec<&str>>();
            bar.set_message(format!(".../.../{}", msg.last().unwrap()));
        } else {
            bar.set_message(bar_msg.to_string());
        }

        // Open image and grab its dimensions.
        let img = image::open(&file.path())?;
        let (width, height) = img.dimensions();
        let ratio = width as f64 / height as f64;
        let afile = file.clone();
        rayon::spawn(move || {
            // Generate a thumbnail and blur if they doesn't already exist.
            let stem = afile
                .path()
                .file_stem()
                .and_then(|p| p.to_str())
                .expect("File stem unwrap issue.");
            let ext = afile
                .path()
                .extension()
                .and_then(|p| p.to_str())
                .expect("Extension unwrap issue.");
            let thumbnail = format!("{}_small.{}", stem, ext);
            let blur = format!("{}_blur.{}", stem, ext);
            let thumb_width = if ratio < 3.0 { 500 } else { 900 };
            if !afile.path().with_file_name(&thumbnail).exists()
                && !afile.path().with_file_name(&blur).exists()
            {
                let thumb = img.resize(thumb_width, 500, Lanczos3);
                thumb
                    .save(afile.path().with_file_name(thumbnail))
                    .expect("Failed to save thumbnail.");
                thumb
                    .blur(30.0)
                    .save(afile.path().with_file_name(blur))
                    .expect("Failed to save blur.");
            } else if !afile.path().with_file_name(&thumbnail).exists() {
                img.resize(thumb_width, 500, Lanczos3)
                    .save(afile.path().with_file_name(thumbnail))
                    .expect("Failed to save thumbnail.");
            } else if !afile.path().with_file_name(&blur).exists() {
                img.resize(thumb_width, 500, Lanczos3)
                    .blur(30.0)
                    .save(afile.path().with_file_name(blur))
                    .expect("Failed to save blur.");
            }
        });

        // Get image decription if it exists, create file if not.
        let mut description = String::new();
        let _ = File::open(file.path().with_extension("desc"))
            .or_else(|_| File::create(file.path().with_extension("desc")))
            .and_then(|mut f| f.read_to_string(&mut description));

        // Build a manifest of all files. We do this entirely each time as descriptions or filenames may have changed.
        let mut path_iter = file.path().strip_prefix("../dist/gallery/")?.iter().rev();

        let name = path_iter
            .next()
            .and_then(|p| p.to_str())
            .ok_or(anyhow!("File name unwrap issue."))?;
        let location_str = path_iter
            .next()
            .and_then(|p| p.to_str())
            .ok_or(anyhow!("Location unwrap issue."))?;
        let location = to_location_identfier_string(&location_str).parse::<Location>()?;
        let _country = path_iter.next();
        let month = path_iter
            .next()
            .and_then(|p| p.to_str())
            .ok_or(anyhow!("Month unwrap issue."))?
            .parse::<Month>()?;
        let year = path_iter
            .next()
            .and_then(|p| p.to_str())
            .ok_or(anyhow!("Year unwrap issue."))?;

        if idx != 0 {
            write!(
                manifest,
                "    , Image \"{}\" (Date {} {:?}) {:?} {:.3} \"{}\"\n",
                name,
                year,
                month,
                location,
                ratio,
                description.trim()
            )?;
        } else {
            write!(
                manifest,
                "    [ Image \"{}\" (Date {} {:?}) {:?} {:.3} \"{}\"\n",
                name,
                year,
                month,
                location,
                ratio,
                description.trim()
            )?;
        }
    }
    writeln!(manifest, "    ]")?;
    bar.finish();

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl FromStr for Month {
    type Err = Error;

    fn from_str(s: &str) -> Result<Month, Error> {
        match s {
            "01" => Ok(Month::Jan),
            "02" => Ok(Month::Feb),
            "03" => Ok(Month::Mar),
            "04" => Ok(Month::Apr),
            "05" => Ok(Month::May),
            "06" => Ok(Month::Jun),
            "07" => Ok(Month::Jul),
            "08" => Ok(Month::Aug),
            "09" => Ok(Month::Sep),
            "10" => Ok(Month::Oct),
            "11" => Ok(Month::Nov),
            "12" => Ok(Month::Dec),
            err => Err(anyhow!("{} makes no sense to be a month.", err)),
        }
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
