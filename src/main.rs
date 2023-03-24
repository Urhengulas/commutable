use std::fmt::Display;

use serde::Deserialize;

const BASE_URL: &str = "https://maps.googleapis.com/maps/api/directions/json";
const API_KEY: &str = include_str!("../maps-api-key");

#[derive(Debug)]
struct Location(String);

impl From<&str> for Location {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

enum Transport {
    Car,
    CarPool,
    Cycling,
    Transit,
    Walking,
}

impl Transport {
    fn mode(&self) -> &str {
        match self {
            Transport::Car | Transport::CarPool => "driving",
            Transport::Cycling => "bicycling",
            Transport::Transit => "transit",
            Transport::Walking => "walking",
        }
    }

    /// g CO2 per km
    fn co2(&self) -> u32 {
        match self {
            Transport::Car | Transport::CarPool => 118,
            Transport::Cycling | Transport::Walking => 0,
            Transport::Transit => 86,
        }
    }
}

/// Measure route from `origin` to `destination`.
///
/// `origin` and `destination` are expected to represent addresses.
///
/// Returns the `distance` in meters and `duration` in seconds.
fn measure_route(origin: Location, destination: Location, transport: Transport) -> (u32, u32) {
    #[derive(Debug, Deserialize)]
    struct Response {
        routes: Vec<Route>,
    }

    #[derive(Debug, Deserialize)]
    struct Route {
        legs: Vec<Leg>,
    }

    #[derive(Debug, Deserialize)]
    struct Leg {
        distance: Entry,
        duration: Entry,
    }

    #[derive(Debug, Deserialize)]
    struct Entry {
        value: u32,
    }

    let url = format!(
        "{BASE_URL}?key={API_KEY}&origin={origin}&destination={destination}&mode={}",
        transport.mode()
    );

    let res: Response = reqwest::blocking::get(url).unwrap().json().unwrap();
    let leg = &res.routes[0].legs[0];
    (leg.distance.value, leg.duration.value)
}

fn main() {
    let home = "Richardstraße 64, 12055".into();
    let work = "Am Friedrichshain 20D, 10407 Berlin".into();
    println!("Measure route from \"{home}\" to \"{work}\".");

    let (distance, duration) = measure_route(home, work, Transport::Car);
    println!("- Distance: {distance} meters");
    println!("- Duration: {} minutes", duration / 60);
}
