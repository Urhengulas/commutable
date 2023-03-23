from typing import Dict, Tuple

"""Map commute type to gramm of co2 per km"""
TRANSPORT_TYPES: Dict[str, Tuple[float, int]] = {
    "BIKE": (0.0, 1),
    "BUS": (86.5, 1),
    "CAR": (118.0, 1),
    "CAR_POOL": (118.0, 2),
}


def calculate_route(commute_type) -> int:
    """
    Calculate the route.

    Returns the distance in meter.
    """
    distance = 20_000
    if commute_type == "CAR_POOL":
        distance *= 1.1
    return distance


def calculate_emission(
    distance: int, emission_factor: int, number_of_people: int
) -> int:
    """
    Calculate the emissions.

    Returns the emissions in gramm of CO2 per person.
    """
    return (distance * emission_factor) / number_of_people


if __name__ == "__main__":
    transport_emissions = {}
    for commute_type, (emission_factor, number_of_people) in TRANSPORT_TYPES.items():
        distance = calculate_route(commute_type)
        total_emission = calculate_emission(distance, emission_factor, number_of_people)
        transport_emissions[commute_type] = total_emission

    car_emissions = transport_emissions["CAR"]
    for commute_type, total_emission in transport_emissions.items():
        savings = 100 - (total_emission / car_emissions * 100)
        print(f"{commute_type}: {total_emission} g CO2 ({savings:.2f}% reduction)")
