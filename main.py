# std
from typing import Dict, Tuple

# local
from maps import measure_route

"""Map commute type to gramm of co2 per km and amount of people"""
TRANSPORT_TYPES: Dict[str, Tuple[float, int]] = {
    "BIKE": (0.0, 1),
    "BUS": (86.5, 1),
    "CAR": (118.0, 1),
    "CAR_POOL": (118.0, 2),
}


def calculate_emission(
    distance: int, emission_factor: int, number_of_people: int
) -> int:
    """
    Calculate the emissions.

    Returns the emissions in gramm of CO2 per person.
    """
    return (distance * emission_factor) / number_of_people


if __name__ == "__main__":
    HOME = "Flutstraße 23, 12439 Berlin"
    WORK = "Am Friedrichshain 20D, 10407 Berlin"

    transport_emissions: Dict[str, Tuple[int, int]] = dict()
    for commute_type, (emission_factor, number_of_people) in TRANSPORT_TYPES.items():
        (distance, duration) = measure_route(HOME, WORK, commute_type)
        total_emission = calculate_emission(distance, emission_factor, number_of_people)
        transport_emissions[commute_type] = (total_emission, duration)

    (car_emissions, _) = transport_emissions["CAR"]
    for commute_type, (total_emission, duration) in transport_emissions.items():
        savings = 100 - (total_emission / car_emissions * 100)
        minutes = int(duration / 60)
        kg = int(total_emission / 1000)
        print(
            f"{commute_type} takes {minutes} min and produces {kg} kg of CO2. That is a {savings:.2f}% reduction compared to taking the car."
        )
