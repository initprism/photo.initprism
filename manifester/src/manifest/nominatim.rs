use anyhow::{bail, Result};
use reqwest::header::USER_AGENT;

static NOMINATIM_ENDPOINT: &str = "http://nominatim.openstreetmap.org";
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Deserialize, Debug)]
pub struct Nominatim {
    lat: String,
    lon: String,
}

impl Nominatim {
    pub fn search(place_name: &str) -> Result<Self> {
        let params = vec![("format", "jsonv2"), ("q", place_name), ("limit", "1")];
        let query_string = Self::get_query_string(params);

        let url = format!("{}/search?{}", NOMINATIM_ENDPOINT, query_string);
        let client = reqwest::blocking::Client::new();

        let res = client
            .get(&url)
            .header(USER_AGENT, format!("{} v{} - {}", NAME, VERSION, AUTHORS))
            .send()?;

        let mut results = res.json::<Vec<Self>>()?;
        results.reverse();

        match results.pop() {
            Some(latlon) => Ok(latlon),
            None => bail!("Search for {} did not find coordinates", place_name),
        }
    }

    pub fn lat(&self) -> String {
        self.lat.to_string()
    }

    pub fn lon(&self) -> String {
        self.lon.to_string()
    }

    fn get_query_string(params: Vec<(&str, &str)>) -> String {
        let pairs: Vec<String> = params
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        pairs.join("&")
    }
}
