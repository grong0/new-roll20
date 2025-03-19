from box import Box

def get_race(races: Box, race_name: str, race_source: str) -> Box:
    for race in races.race:
        if race.name == race_name and race.source == race_source:
            return race
    print("no race found")
    return Box({})

races = Box().from_json(filename="./data/raw/races.json")

for race in races.race:
    if "speed" not in race.keys():
        print(get_race(races, race._copy.name, race._copy.source).speed)
    else:
        print(race.speed)
