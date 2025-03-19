import datetime
import json
from enum import StrEnum
from typing import Optional
from src.DAO.MiscClasses import (
    Source,
    Speed,
    Ability,
    HeightAndWeight,
    Age,
    SkillProficiencies,
    LanguageProficiencies,
    ToolProficiencies,
    WeaponProficiencies,
    ArmorProficiencies,
    Resist,
    AdditionalSpells,
)


class Race:
    name: str
    source: Source
    srd: Optional[bool] = None
    basic_rules: Optional[bool] = None
    other_sources: list[Source] = []
    reprinted_as: Optional[str] = None
    copy: Optional[dict[str, any]] = {}
    lineage: Optional[str] = None
    creature_types: list[str] = []
    creature_type_tags: list[str] = []
    size: list[str] = []
    speed: Speed
    ability: Ability
    height_and_weight: HeightAndWeight
    age: Age
    darkvision: int = 0
    trait_tags: list[str] = []
    skill_proficiencies: SkillProficiencies
    language_proficiencies: LanguageProficiencies
    tool_proficiencies: ToolProficiencies
    weapon_proficiencies: WeaponProficiencies
    armor_proficiencies: ArmorProficiencies
    resist: Resist
    additional_spells: AdditionalSpells
    immune: list[str] = []
    conditionImmune: list[str] = []
    entries: list[dict[str, list[str] | str]] | str = []
    has_fluff: bool = False
    has_fluff_images: bool = False
    versions: list[dict[str, any]] = []

    def __init__(self, object: dict[str, any]) -> None:
        self.name = object["name"]
        self.source = object["source"]
        self.srd = object["srd"] if "srd" in object.keys() else self.srd
        self.basic_rules = (
            object["basicRules"] if "basicRules" in object.keys() else self.basic_rules
        )
        self.other_sources = (
            object["otherSources"]
            if "otherSources" in object.keys()
            else self.other_sources
        )
        self.reprinted_as = (
            object["reprintedAs"]
            if "reprintedAs" in object.keys()
            else self.reprinted_as
        )
        self.copy = object["_copy"] if "_copy" in object.keys() else self.copy
        self.lineage = object["lineage"] if "lineage" in object.keys() else self.lineage
        self.creature_types = (
            object["creatureTypes"]
            if "creatureTypes" in object.keys()
            else self.creature_types
        )
        self.creature_type_tags = (
            object["createTypeTags"]
            if "createTypeTags" in object.keys()
            else self.creature_type_tags
        )
        self.size = (
            object["size"] if "size" in object.keys() else self.size
        )  # leave for copy
        self.speed = Speed(object["speed"]) if "speed" in object.keys() else Speed(None)
        self.ability = (
            Ability(object["ability"][0])
            if "ability" in object.keys()
            else Ability(None)
        )
        self.height_and_weight = (
            HeightAndWeight(object["heightAndWeight"])
            if "heightAndWeight" in object.keys()
            else HeightAndWeight(None)
        )
        self.age = (
            Age(object["age"]) if "age" in object.keys() else HeightAndWeight(None)
        )
        self.darkvision = (
            object["darkvision"] if "darkvision" in object.keys() else self.darkvision
        )
        self.trait_tags = (
            object["traitTags"] if "immune" in object.keys() else self.immune
        )
        self.skill_proficiencies = (
            SkillProficiencies(object["skillProficiencies"][0])
            if "skillProficiencies" in object.keys()
            else SkillProficiencies(None)
        )
        self.language_proficiencies = (
            LanguageProficiencies(object["languageProficiencies"][0])
            if "languageProficiencies" in object.keys()
            else LanguageProficiencies(None)
        )
        self.tool_proficiencies = (
            ToolProficiencies(object["toolProficiencies"])
            if "toolProficiencies" in object.keys()
            else ToolProficiencies(None)
        )
        self.weapon_proficiencies = (
            WeaponProficiencies(object["weaponProficiencies"][0])
            if "weaponProficiencies" in object.keys()
            else WeaponProficiencies(None)
        )
        self.armor_proficiencies = (
            ArmorProficiencies(object["armorProficiencies"][0])
            if "armorProficiencies" in object.keys()
            else ArmorProficiencies(None)
        )
        self.resist = (
            Resist(object["resist"]) if "resist" in object.keys() else Resist(None)
        )
        self.additional_spells = (
            AdditionalSpells(object["additionalSpells"])
            if "additionalSpells" in object.keys()
            else AdditionalSpells(None)
        )
        self.immune = object["immune"] if "immune" in object.keys() else self.immune
        self.conditionImmune = (
            object["conditionImmune"] if "immune" in object.keys() else self.immune
        )
        self.entries = object["entries"] if "entries" in object.keys() else self.entries
        self.has_fluff = (
            object["hasFluff"] if "hasFluff" in object.keys() else self.has_fluff
        )
        self.has_fluff_images = (
            object["hasFluffImages"]
            if "hasFluffImages" in object.keys()
            else self.has_fluff_images
        )
        self.versions = (
            object["_versions"] if "_versions" in object.keys() else self.versions
        )

        self.__dict__["source"] = self.source.__dict__
        self.__dict__["speed"] = self.speed.__dict__
        self.__dict__["ability"] = self.ability.__dict__
        self.__dict__["height_and_weight"] = self.height_and_weight.__dict__
        self.__dict__["age"] = self.age.__dict__
        self.__dict__["skill_proficiencies"] = self.skill_proficiencies.__dict__
        self.__dict__["language_proficiencies"] = self.language_proficiencies.__dict__
        self.__dict__["tool_proficiencies"] = self.tool_proficiencies.__dict__
        self.__dict__["weapon_proficiencies"] = self.weapon_proficiencies.__dict__
        self.__dict__["armor_proficiencies"] = self.armor_proficiencies.__dict__
        self.__dict__["resist"] = self.resist.__dict__
        self.__dict__["additional_spells"] = self.additional_spells.__dict__


class Races:
    races: dict[str, Race]

    def __init__(self) -> None:
        self.races = {}
        with open("data/raw/races.json") as f:
            object = json.load(f)

            for race in object["race"]:
                self.races[race["name"]] = Race(race)
                # print(
                #     f"finished {self.races[-1].name} | {len(self.races)}/{len(object['race'])}"
                # )

    def get_race(self, name: str) -> Optional[Race]:
        try:
            return self.races[name]
        except:
            return None


if __name__ == "__main__":
    times = []
    count = 1000
    for _ in range(0, count):
        start = datetime.datetime.now()
        races = Races()
        end = datetime.datetime.now()
        times.append((end - start).microseconds)
        del races

    average_time = sum(times) / count

    print(f"average time was {average_time} microseconds")
