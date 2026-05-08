//! Handle getting the location

use crate::errors::SunshineError;

use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) struct Location {
	pub(crate) latitude: f64,
	pub(crate) longitude: f64,
}

#[allow(unused)]
#[derive(Clone, Debug, Deserialize)]
struct OSMJsonResponse {
	place_id: u64,
	osm_type: String,
	osm_id: u64,
	lat: String,
	lon: String,
	class: String,
	place_rank: u64,
	importance: f64,
	addresstype: String,
	name: String,
	display_name: String,
	boundingbox: Vec<String>,
}

#[allow(non_snake_case, unused)]
#[derive(Clone, Debug, Deserialize)]
struct GeoIpJsonResponse {
	ip: String,
	city: String,
	country: String,
	countryRegion: String,
	continent: String,
	latitude: String,
	longitude: String,
	timezone: String,
	postalCode: String,
	region: String,
}

fn location_from_coords(coords: &str) -> Result<Location, SunshineError> {
	let coords: Vec<&str> = coords.split(' ').collect();

	let latitude = match coords.get(0) {
		Some(value) => Some(value.parse()),
		None => return Err(SunshineError::MalformedLocationString),
	};
	let longitude = match coords.get(1) {
		Some(value) => Some(value.parse()),
		None => return Err(SunshineError::MalformedLocationString),
	};

	if latitude.is_some() && longitude.is_some() {
		Ok(Location { latitude: latitude.unwrap()?, longitude: longitude.unwrap()? })
	} else {
		Err(SunshineError::MalformedLocationString)
	}
}

fn location_from_ip() -> Result<Location, SunshineError> {
	let api_url = "https://geo.kamero.ai/api/geo";

	let url = Url::parse(api_url).expect("couldn't parse `api_url`");
	let client = reqwest::blocking::Client::new();
	let request = client
		.get(url)
		.header(
			reqwest::header::USER_AGENT,
			"sunshine/0.1.0 (https://github.com/ArkhamCookie/sunshine",
		)
		.build()?;

	let response = client.execute(request)?;
	let body = response.json::<GeoIpJsonResponse>()?;

	let latitude = &body.latitude.parse::<f64>()?;
	let longitude = &body.longitude.parse::<f64>()?;

	Ok(Location { latitude: *latitude, longitude: *longitude })
}

fn location_from_name(name: &str) -> Result<Location, SunshineError> {
	let api_url = "https://nominatim.openstreetmap.org";

	let url = Url::parse(api_url).expect("couldn't parse `api_url`").join("search").expect("couldn't join `search` to url");
	let client = reqwest::blocking::Client::new();
	let request = client
		.get(url)
		.header(
			reqwest::header::USER_AGENT,
			"sunshine/0.1.0 (https://github.com/ArkhamCookie/sunshine)",
		)
		.query(&[("q", name), ("format", "json")])
		.build()?;

	let response = client.execute(request)?;
	let body = response.json::<Vec<OSMJsonResponse>>()?;
	let location = &body[0];

	let latitude = &location.lat.parse::<f64>()?;
	let longitude = &location.lon.parse::<f64>()?;
	// let name = &location.display_name;

	Ok(Location { latitude: *latitude, longitude: *longitude })
}

pub(crate) fn get_location(location: String) -> Result<Location, SunshineError> {
	match &location[..1] {
		"@" => location_from_coords(&location[1..]),
		"#" => location_from_name(&location[1..]),
		"." => location_from_ip(),
		_ => Err(SunshineError::MalformedLocationString),
	}
}
