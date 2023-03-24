use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};

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
    Cycle,
    Transit(Option<TransitMode>),
    Walk,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
enum Propulsion {
    Diesel,
    Electric,
    Gas,
}

impl FromStr for Propulsion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "diesel" | "Diesel" => Self::Diesel,
            "electric" | "Electric" => Self::Electric,
            "gas" | "Gas" => Self::Gas,
            _ => unreachable!(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
enum CarSize {
    Small,
    Medium,
    Big,
}

impl FromStr for CarSize {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "small" | "Small" => Self::Small,
            "medium" | "Medium" => Self::Medium,
            "big" | "Big" => Self::Big,
            _ => unreachable!(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
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
            Transport::Cycle => "bicycling",
            Transport::Transit(_) => "transit",
            Transport::Walk => "walking",
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
            Transport::Cycle | Transport::Walk => 0,
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
                Transport::Cycle => "Cycling",
                Transport::Transit(_) => "Public transport",
                Transport::Walk => "Walking",
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
async fn measure_route(
    origin: &Location,
    destination: &Location,
    transport: &mut Transport,
) -> (u32, u32) {
    // the structs are needed to deserialize the response
    #[derive(Debug, Deserialize)]
    struct MapsResponse {
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

    let mut url = format!(
        "{}?key={}&origin={}&destination={}&mode={}&departure_time={}",
        BASE_URL,
        API_KEY,
        origin,
        destination,
        transport.mode(),
        1679896800, // Mon Mar 27 2023 08:00:00 GMT+0200 (CEST)
    );

    if let Transport::CarPool { stopover, .. } = transport {
        url.push_str(format!("&waypoints=via:{stopover}").as_str())
    }

    let json: MapsResponse = reqwest::get(&url).await.unwrap().json().await.unwrap();
    let leg = &json.routes[0].legs[0];

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
    (distance * transport.co2()) / 1000 / number_of_people
}

#[tokio::main]
async fn main() {
    let car = warp::path!("car")
        .and(warp::query::<RouteQuery>())
        .and(warp::query::<CarQuery>())
        .and_then(handle_car);
    let car_pool = warp::path!("carpool")
        .and(warp::query::<RouteQuery>())
        .and(warp::query::<CarQuery>())
        .and(warp::query::<CarPoolQuery>())
        .and_then(handle_carpool);
    let cycle = warp::path!("cycle")
        .and(warp::query::<RouteQuery>())
        .and_then(|route_query| handle_transport(route_query, Transport::Cycle));
    let transit = warp::path!("transit")
        .and(warp::query::<RouteQuery>())
        .and_then(|route_query| handle_transport(route_query, Transport::Transit(None)));
    let walk = warp::path!("walk")
        .and(warp::query::<RouteQuery>())
        .and_then(|route_query| handle_transport(route_query, Transport::Walk));
    let router = car.or(car_pool).or(cycle).or(transit).or(walk);

    println!("Started listening on http://127.0.0.1:3030.");
    warp::serve(router).run(([127, 0, 0, 1], 3030)).await;
}

#[derive(Debug, Deserialize)]
struct RouteQuery {
    origin: Location,
    destination: Location,
}

#[derive(Debug, Deserialize)]
struct CarQuery {
    propulsion: String,
    size: String,
}

#[derive(Debug, Deserialize)]
struct CarPoolQuery {
    stopover: Location,
}

#[derive(Debug, Serialize)]
struct ApiResponse {
    distance: u32,
    duration: u32,
    emissions: u32,
}

async fn handle_car(route_query: RouteQuery, car_query: CarQuery) -> Result<impl Reply, Rejection> {
    let propulsion = car_query.propulsion.parse::<Propulsion>().unwrap();
    let size = car_query.size.parse::<CarSize>().unwrap();
    let transport = Transport::Car { propulsion, size };
    handle_transport(route_query, transport).await
}

async fn handle_carpool(
    route_query: RouteQuery,
    car_query: CarQuery,
    carpool_query: CarPoolQuery,
) -> Result<impl Reply, Rejection> {
    let propulsion = car_query.propulsion.parse::<Propulsion>().unwrap();
    let size = car_query.size.parse::<CarSize>().unwrap();
    let transport = Transport::CarPool {
        propulsion,
        size,
        stopover: carpool_query.stopover,
    };
    handle_transport(route_query, transport).await
}

async fn handle_transport(
    route_query: RouteQuery,
    mut transport: Transport,
) -> Result<impl Reply, Rejection> {
    let (distance, duration) = measure_route(
        &route_query.origin,
        &route_query.destination,
        &mut transport,
    )
    .await;
    let emissions = calculate_emission(distance, &transport);
    Ok(warp::reply::json(&ApiResponse {
        distance,
        duration,
        emissions,
    }))
}

async fn experiment_function() {
    let home = "Hamburg".into();
    let work = "Berlin".into();
    let mut transport = Transport::Car {
        propulsion: Propulsion::Electric,
        size: CarSize::Medium,
    };
    let (distance, duration) = measure_route(&home, &work, &mut transport).await;
    let emissions = calculate_emission(distance, &transport);
    println!("{} km", distance / 1000);
    println!("{} minutes", duration / 60);
    println!("{} kg CO2", emissions / 1000);
}
