from typing import Optional
from Lineage import Lineage
from Race import Race


class Lineages:
    lineages: dict[str, Lineage]

    def __init__(self) -> None:
        self.lineages = {}

    def add_lineage(self, lineage: Lineage) -> None:
        self.lineages[lineage.name] = lineage

    def get_race_linage(self, race: Race) -> Optional[Lineage]:
        for key in self.lineages:
            if self.lineages[key].contains(race):
                return self.lineages[key]
    
    def get_race_name_lineage(self, name: str) -> Optional[Lineage]:
        for key in self.lineages:
            if name in self.lineages[key].races.keys():
                return self.lineages[key]
    
    def get_race(self, name: str) -> Optional[Race]:
        lineage = self.get_race_name_lineage(name)
        if lineage != None:
            return lineage.get_race(name)
        return None

    def get_as_dict(self) -> dict[str, dict[str, dict[str, str | bool | dict[str, dict[str, str | list[str] | list[dict[str, str| int | dict[str, str | list[str] | list[list[str]] | None]]]]]]]]:
        self_dict = {}
        for key in self.lineages:
            self_dict[key] = self.lineages[key].get_as_dict()
        return self_dict
