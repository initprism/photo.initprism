use anyhow::{bail, Result};
use std::collections::BTreeMap;
use std::fmt;
use strum_macros::EnumString;

use crate::utils::to_location_identfier_string;

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    #[serde(rename = "type")]
    pub type_: String,
    pub properties: Properties,
    pub geometry: Geometry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub type_: String,
    pub coordinates: Coordinates,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Coordinates {
    Point(Vec<f32>),
    LineString(Vec<Vec<f32>>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureCollection {
    #[serde(rename = "type")]
    pub type_: String,
    pub features: Vec<Feature>,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Country {
    Korea
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Country {
    pub fn name(&self) -> String {
        let mut name: Vec<char> = Vec::new();

        for (idx, c) in self.to_string().char_indices() {
            if idx > 0 && c.is_uppercase() {
                name.push(' ');
            }
            name.push(c);
        }
        String::from_iter(name)
    }

    pub fn code(&self, cca3: &BTreeMap<String, String>) -> Result<String> {
        match cca3.get(&self.name()) {
            Some(code) => Ok(code.to_string()),
            None => bail!("{} does not exist in cca3.json", self.name()),
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, EnumString, Serialize, Deserialize)]
pub enum Location {
    Local,
    Seoul,
    Andong,
    Jeju,
    Cheongju,
    Danyang,
    Gangneung,
    Yeongdeok,
    Jinju,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Location {
    pub fn name(&self) -> String {
        let mut name: Vec<char> = Vec::new();

        for (idx, c) in self.to_string().char_indices() {
            if idx > 0 && c.is_uppercase() {
                name.push(' ');
            }
            name.push(c);
        }

        String::from_iter(name)
    }

    pub fn feature_coordinates(&self, features: &[Feature]) -> Result<Vec<f32>> {
        for feature in features {
            if to_location_identfier_string(&feature.properties.name) == self.to_string() {
                match &feature.geometry.coordinates {
                    Coordinates::Point(coords) => return Ok(coords.clone()),
                    _ => {
                        bail!("{} does not have Point coordinates.", self);
                    }
                };
            }
        }
        bail!("Could not find coordinates for {}.", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trip {
    pub name: String,
    pub description: String,
    pub cities: Vec<Location>,
    pub dates: Vec<String>,
}

impl Trip {
    pub fn id_string(&self) -> String {
        let mut id = self.description.to_string();

        id.retain(|c| c != ' ' && c != '/');
        id
    }
}
