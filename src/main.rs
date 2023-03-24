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
    Sbahn,
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

    /// g CO2 eq per km
    fn co2(&self) -> u32 {
        match self {
            Transport::CarPool {
                propulsion, size, ..
            }
            | Transport::Car { propulsion, size } => match (propulsion, size) {
                (Propulsion::Diesel, CarSize::Small) => 240,
                (Propulsion::Diesel, CarSize::Medium) => 310,
                (Propulsion::Diesel, CarSize::Big) => 390,
                (Propulsion::Electric, CarSize::Small) => 160,
                (Propulsion::Electric, CarSize::Medium) => 200,
                (Propulsion::Electric, CarSize::Big) => 240,
                (Propulsion::Gas, CarSize::Small) => 280,
                (Propulsion::Gas, CarSize::Medium) => 340,
                (Propulsion::Gas, CarSize::Big) => 410,
            },
            Transport::Cycling | Transport::Walking => 0,
            Transport::Transit(Some(transit_mode)) => match transit_mode {
                TransitMode::Bus => 108,
                TransitMode::Train => 93,
                TransitMode::Sbahn | TransitMode::Subway | TransitMode::Tram => 80,
            },
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
            "COMMUTER_TRAIN" => Some(Self::Sbahn),
            "SUBWAY" => Some(Self::Subway),
            "HEAVY_RAIL"
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
    // the structs are needed to deserialize the response
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
        "{}?key={}&origin={}&destination={}&mode={}&departure_time={}",
        BASE_URL,
        API_KEY,
        origin,
        destination,
        transport.mode(),
        1679896800, // Mon Mar 27 2023 08:00:00 GMT+0200 (CEST)
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
            "{} takes {} minutes and produces {} kg of CO2 equivalent per Person. That is a {:.2}% reduction compared to taking the car.",
            transport,
            duration / 60,
            emissions / 1000,
            savings,
        )
    }
}
