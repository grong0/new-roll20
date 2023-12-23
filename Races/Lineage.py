from typing import Optional
from Race import Race


class Lineage:
    name: str
    setting_specific: bool
    races: dict[str, Race]

    def __init__(
        self, name: str, setting_specific: bool, races: list[dict[str, str]]
    ) -> None:
        self.name = name
        self.setting_specific = setting_specific
        self.races = {}
        for race in races:
            self.races[race["name"]] = Race(race["name"], race["link"])
            if "source" in race.keys():
                self.races[race["name"]].source = race["source"]
    
    def get_race(self, name: str) -> Optional[Race]:
        return self.races[name]

    def add_race(self, race: Race) -> None:
        self.races[race.name] = race

    def contains(self, race: Race) -> bool:
        return race in self.races

    def get_as_dict(self) -> dict[str, str | bool | dict[str, dict[str, str | list[str] | list[dict[str, str| int | dict[str, str | list[str] | list[list[str]] | None]]]]]]:
        dict_races: dict[str, dict[str, str | list[str] | list[dict[str, str| int | dict[str, str | list[str] | list[list[str]] | None]]]]] = {}
        for key in self.races:
            dict_races[key] = self.races[key].get_as_dict()
        return {
            "name": self.name,
            "setting_specific": self.setting_specific,
            "races": dict_races,
        }
