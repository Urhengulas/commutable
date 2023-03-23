# std
import json
from typing import Tuple

# 3rd-party
import requests

api_key_file = open("maps-api-key")
API_KEY = api_key_file.readline()
api_key_file.close()

ENDPOINT = f"https://maps.googleapis.com/maps/api/directions/json?key={API_KEY}"


def measure_route(origin: str, destination: str, _commute_type: str) -> Tuple[int, int]:
    """
    Measure route from `origin` to `destination`.

    `origin` and `destination` are expected to represent addresses.

    Returns the `distance` in meters and `duration` in seconds.
    """

    # Call the API
    url = f"{ENDPOINT}&origin={origin}&destination={destination}"
    response = requests.get(url)
    data = json.loads(response.text)

    # Extract the route information
    leg = data["routes"][0]["legs"][0]
    distance = leg["distance"]["value"]
    duration = leg["duration"]["value"]

    # TODO: get the actual distance per commute type
    if _commute_type == "CAR_POOL":
        distance *= 1.1

    return (distance, duration)


if __name__ == "__main__":
    origin = "Flutstraße 23, 12439 Berlin"
    destination = "Am Friedrichshain 20D, 10407 Berlin"
    (distance, duration) = measure_route(origin, destination, "CAR")

    # Print the route information
    print(f"Measure route from '{origin}' to '{destination}'.")
    print(f"- Distance: {distance} meters")
    print(f"- Duration: {duration} seconds")
