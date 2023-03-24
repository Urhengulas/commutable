use std::{collections::HashMap, fmt::Display};

use serde::Deserialize;

const BASE_URL: &str = "https://maps.googleapis.com/maps/api/directions/json";
const API_KEY: &str = include_str!("../maps-api-key");

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Transport {
    Car {
        propulsion: Propulsion,
        size: CarSize,
    },
    CarPool {
        propulsion: Propulsion,
        size: CarSize,
        stopover: Location,
    },
    Cycling,
    Transit(Option<TransitMode>),
    Walking,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Propulsion {
    Diesel,
    Electric,
    Gas,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum CarSize {
    Small,
    Medium,
    Big,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Location(String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum TransitMode {
    Bus,
    Subway,
    Train,
    Tram,
}

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

impl Transport {
    fn mode(&self) -> &str {
        match self {
            Transport::Car { .. } | Transport::CarPool { .. } => "driving",
            Transport::Cycling => "bicycling",
            Transport::Transit(_) => "transit",
            Transport::Walking => "walking",
        }
    }

    /// g CO2 per km
    fn co2(&self) -> u32 {
        match self {
            // TODO: consider car size
            Transport::Car { .. } | Transport::CarPool { .. } => 118,
            Transport::Cycling | Transport::Walking => 0,
            Transport::Transit(Some(_)) => 86,
            Transport::Transit(None) => {
                unreachable!("transit mode needs to be set to calculate CO2")
            }
        }
    }
}

impl Display for Transport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Transport::Car { .. } => "Driving",
                Transport::CarPool { .. } => "Carpooling",
                Transport::Cycling => "Cycling",
                Transport::Transit(_) => "Public transport",
                Transport::Walking => "Walking",
            }
        )
    }
}

impl TransitMode {
    fn parse_vehicle_type(s: &str) -> Option<Self> {
        match s {
            "BUS" | "INTERCITY_BUS" | "TROLLEYBUS" => Some(Self::Bus),
            "SUBWAY" => Some(Self::Subway),
            "COMMUTER_TRAIN"
            | "HEAVY_RAIL"
            | "HIGH_SPEED_TRAIN"
            | "LONG_DISTANCE_TRAIN"
            | "METRO_RAIL"
            | "MONORAIL"
            | "RAIL" => Some(Self::Train),
            "TRAM" => Some(Self::Tram),
            "CABLE_CAR" | "FERRY" | "FUNICULAR" | "GONDOLA_LIFT" | "OTHER" | "SHARE_TAXI" => None,
            _ => unreachable!(),
        }
    }
}

/// Measure route from `origin` to `destination`.
///
/// `origin` and `destination` are expected to represent addresses.
///
/// Returns the `distance` in meters and `duration` in seconds.
fn measure_route(
    origin: &Location,
    destination: &Location,
    transport: &mut Transport,
) -> (u32, u32) {
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
        steps: Vec<Step>,
    }

    #[derive(Debug, Deserialize)]
    struct Entry {
        value: u32,
    }

    #[derive(Debug, Deserialize)]
    struct Step {
        transit_details: Option<TransitDetails>,
    }

    #[derive(Clone, Debug, Deserialize)]
    struct TransitDetails {
        line: Line,
    }

    #[derive(Clone, Debug, Deserialize)]
    struct Line {
        vehicle: Vehicle,
    }

    #[derive(Clone, Debug, Deserialize)]
    struct Vehicle {
        r#type: String,
    }

    let url = format!(
        "{BASE_URL}?key={API_KEY}&origin={origin}&destination={destination}&mode={}",
        transport.mode()
    );

    let res: Response = reqwest::blocking::get(&url).unwrap().json().unwrap();
    let leg = &res.routes[0].legs[0];

    if let Transport::Transit(transit_mode) = transport {
        let transit_modes = leg
            .steps
            .iter()
            .filter_map(|step| step.transit_details.clone())
            .map(|transit_details| transit_details.line.vehicle.r#type)
            .filter_map(|vehicle_type| TransitMode::parse_vehicle_type(&vehicle_type))
            .collect::<Vec<_>>();
        assert!(transit_modes.len() > 0);
        dbg!(&transit_modes);

        // FIXME: atm we just take the first transit mode for the whole trip, but we
        // should calculate it for each step
        *transit_mode = Some(transit_modes[0].clone());
    }

    (leg.distance.value, leg.duration.value)
}

/// Calculate the emissions.
///
/// Returns the emissions in gramm of CO2 per person.
fn calculate_emission(distance: u32, transport: &Transport) -> u32 {
    let number_of_people = match transport {
        Transport::CarPool { .. } => 2,
        _ => 1,
    };
    (distance * transport.co2()) / number_of_people
}

fn main() {
    let home = "Richardstraße 64, 12055".into();
    let work = "Am Friedrichshain 20D, 10407 Berlin".into();
    println!("Measure route from \"{home}\" to \"{work}\".");

    let car = Transport::Car {
        propulsion: Propulsion::Diesel,
        size: CarSize::Medium,
    };
    let calculation = [
        car.clone(),
        Transport::CarPool {
            propulsion: Propulsion::Diesel,
            size: CarSize::Medium,
            stopover: "Lohmühlenstraße 65, 12435 Berlin".into(),
        },
        Transport::Cycling,
        Transport::Transit(None),
        Transport::Walking,
    ]
    .into_iter()
    .map(|mut transport| {
        let (distance, duration) = measure_route(&home, &work, &mut transport);
        let emissions = calculate_emission(distance, &transport);
        (transport, [duration, emissions])
    })
    .collect::<HashMap<_, _>>();

    let [_, car_emission] = calculation.get(&car).unwrap();

    for (transport, [duration, emissions]) in &calculation {
        let savings = 100 - (100 * emissions / car_emission);
        println!(
            "{transport} takes {} minutes and produces {} kg of CO2. That is a {savings:.2}% reduction compared to taking the car.",
            duration / 60,
            emissions / 1000
        )
    }
}
