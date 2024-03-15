from enum import StrEnum
import json
from typing import Optional


class Speed:
    walk: int
    fly: Optional[int] = None
    swim: Optional[int] = None
    climb: Optional[int] = None

    def __init__(self, object: int | dict[str, int]) -> None:
        if object is int:
            self.walk = object
        elif object is dict:
            self.walk = object["walk"]
            self.fly = object["fly"] if "fly" in object.keys() else self.fly
            self.swim = object["swim"] if "swim" in object.keys() else self.swim
            self.climb = object["climb"] if "climb" in object.keys() else self.climb


class Ability:
    strength: int = 0
    dexterity: int = 0
    constitution: int = 0
    intelligence: int = 0
    wisdom: int = 0
    charisma: int = 0
    choose_count: int = 0
    choose_abilities: list[str] = []

    def __init__(self, object: Optional[dict[str, int | dict]]) -> None:
        if object is not None:
            self.strength = object["str"] if "str" in object.keys() else self.strength
            self.dexterity = object["str"] if "str" in object.keys() else self.dexterity
            self.constitution = (
                object["str"] if "str" in object.keys() else self.constitution
            )
            self.intelligence = (
                object["str"] if "str" in object.keys() else self.intelligence
            )
            self.wisdom = object["str"] if "str" in object.keys() else self.wisdom
            self.charisma = object["str"] if "str" in object.keys() else self.charisma
            if "choose" in object.keys():
                self.choose_count = object["choose"]["count"]
                self.choose_abilities = object["choose"]["from"]


class Age:
    mature: Optional[int] = None
    max: Optional[int] = None

    def __init__(self, object: Optional[dict[str, int]]) -> None:
        if object is not None:
            self.mature = object["mature"] if "mature" in object.keys() else self.mature
            self.max = object["max"] if "max" in object.keys() else self.max


class LanguageProficiencies:
    languages: list[str] = []
    any_standard: int = 0
    choose_languages: list[str]
    choose_count: int = 1

    def __init__(
        self, object: Optional[dict[str, bool | int | dict[str, int | list[str]]]]
    ) -> None:
        if object is not None:
            for key in object.keys():
                if type(object[key]) is bool:
                    self.languages.append(key)
                elif type(object[key]) is int:
                    self.any_standard = object[key]
                elif type(object[key]) is dict:
                    self.choose_languages = object["choose"]["from"]
                    self.choose_count = object["choose"]["count"]
                else:
                    print(f"Language proficiency value was not expected")
                    print(f"It was a {type(object[key])} with a value of {object[key]}")


class SkillProficiencies:
    skills: list[str] = []
    choose_skills: list[str] = []
    choose_count: int = 1

    def __init__(
        self, object: Optional[dict[str, bool | dict[str, int | list[str]]]]
    ) -> None:
        if object is not None:
            for key in object.keys():
                if type(object[key]) is bool:
                    self.skills.append(object[key])
                elif type(object[key]) is dict:
                    self.choose_skills = object[key]["from"]
                    self.choose_count = (
                        object[key]["count"]
                        if "count" in object[key].keys()
                        else self.choose_count
                    )
                else:
                    print(f"Skill proficiency  value was not expected")
                    print(f"It was a {type(object[key])} with a value of {object[key]}")


class ToolProficiencies:
    tools: list[str] = []
    choose_any_amount: int = 0

    def __init__(self, object: Optional[list[dict[str, int | str]]]) -> None:
        if object is not None:
            for item in object:
                for key in item.keys():
                    if key == "any":
                        self.choose_any_amount = item["any"]
                    else:
                        self.tools.append(key)


class WeaponProficiencies:
    weapons: list[dict[str, str]] = []
    choose_tool_filter: list[str] = []
    choose_amount: int = 0

    def __init__(self, object: Optional[dict[str, bool]]) -> None:
        if object is not None:
            for key in object.keys():
                if key == "choose":
                    self.choose_tools = object[key]["fromFilter"]
                    self.choose_amount = object[key]["count"]
                else:
                    self.proficiencies.append({
                        "name": key.split("|")[0],
                        "source": key.split("|")[1]
                    })

    def has_proficiency(self, name: str) -> bool:
        for item in self.proficiencies:
            if item["name"] == name:
                return True
        return False


class ArmorProficiencies:
    armor: list[str] = []

    def __init__(self, object: Optional[dict[str, bool]]) -> None:
        if object is not None:
            # not checking whether or not it has any because
            # as of Mar 13, the only race that has a armor
            # proficiency just has a defined armor type
            armor = object.keys()


class Source:
    source: str = ""
    page: Optional[int] = None

    def __init__(
        self,
        object: Optional[dict[str, str | int]] = None,
        source: str = None,
        page: int = None,
    ) -> None:
        if source is not None and page is not None:
            self.source = source
            self.page = page
        elif object is not None:
            self.source = object["source"]
            self.page = object["page"] if "page" in object.keys else self.page


class HeightAndWeight:
    base_height: int = None
    height_mod: str = None
    base_weight: int = None
    weight_mod: str = None

    def __init__(self, object: Optional[dict[str, int | str]]) -> None:
        if object is not None:
            self.base_height = object["base_height"]
            self.height_mod = (
                object["height_mod"]
                if "height_mod" in object.keys()
                else self.height_mod
            )
            self.base_weight = object["base_weight"]
            self.weight_mod = (
                object["weight_mod"]
                if "weight_mod" in object.keys()
                else self.weight_mod
            )


class Resist:
    resists: list[str] = []
    choose_from: list[str] = []

    def __init__(self, object: Optional[list[str | dict[str, list]]]) -> None:
        if object is not None:
            for item in object:
                if type(item) is list:
                    self.resists.append(item)
                elif type(item) is dict[str, list]:
                    self.choose_from = item["from"]


class Reset(StrEnum):
    NEVER = "never"
    REST = "rest"
    DAILY = "daily"


class AdditionalSpells:
    spells: list[dict[str, any]]

    def __init__(self, object: Optional[list[dict[str, dict[str, any]]]]) -> None:
        if object is not None:
            for spell_object in object:
                if "innate" in spell_object.keys():
                    for level in spell_object["innate"].keys():
                        if type(spell_object["innate"][level]) is list:
                            for spell in spell_object["innate"][level]:
                                self.spells.append({
                                    "name": spell,
                                    "ability": spell_object["ability"],
                                    "reset_when": Reset.NEVER,
                                    "aquired_at": int(level)
                                })
                        elif type(spell_object["innate"][level]) is dict:
                            if "daily" in spell_object["innate"][level].keys():
                                for spell in spell_object["innate"][level]["daily"]["1"]:
                                    self.spells.append({
                                        "name": spell,
                                        "ability": spell_object["ability"],
                                        "reset_when": Reset.DAILY,
                                        "aquired_at": int(level)
                                    })
                            else:
                                print("spell_object['innate'][level] was a dict with an unknown key")
                if "known" in spell_object.keys():
                    if type(spell_object["known"]["1"]) is list:
                        for spell in spell_object["known"]["1"]:
                            self.spells.append({
                                "name": spell,
                                "ability": spell_object["ability"],
                                "reset_when": Reset.NEVER,
                                "aquired_at": 0
                            })
                    elif type(spell_object["known"]["1"]) is dict:
                        if "rest" in spell_object["known"]["1"].keys():
                            for spell in spell_object["known"]["1"]["rest"]:
                                self.spells.append({
                                    "name": spell,
                                    "ability": spell_object["ability"],
                                    "reset_when": Reset.REST,
                                    "aquired_at": 0
                                })
                        else:
                            print("spell_object['known'] was a dict with an unknown key")

    def new_spells_at(self, level: int) -> list[str]:
        new_spells: list[str] = []
        for spell in self.spells:
            if spell["aquired_at"] == level:
                new_spells.append(spell["name"])
        return new_spells


class Race:
    name: str
    source: Source
    srd: Optional[bool] = None
    basic_rules: Optional[bool] = None
    other_sources: list[Source] = []
    reprinted_as: Optional[str] = None
    copy: Optional[dict[str, any]]
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
    entries: list[dict[str, list[str] | str]] = []
    has_fluff: bool = False
    has_fluff_images: bool = False
    versions: list[dict[str, any]] = []

    def __init__(self, object: dict[str, any]) -> None:
        self.name = object["name"]
        self.source = object["source"]
        self.srd = object["srd"] if "immune" in object.keys() else self.immune
        self.basic_rules = object["basicRules"] if "immune" in object.keys() else self.immune
        self.other_sources = object["otherSources"] if "immune" in object.keys() else self.immune
        self.reprinted_as = object["reprintedAs"] if "immune" in object.keys() else self.immune
        self.copy = object["_copy"] if "immune" in object.keys() else self.immune
        self.lineage = object["lineage"] if "immune" in object.keys() else self.immune
        self.creature_types = object["creatureTypes"] if "immune" in object.keys() else self.immune
        self.creature_type_tags = object["createTypeTags"] if "immune" in object.keys() else self.immune
        self.size = object["size"] # leave for copy
        self.speed = Speed(object["speed"])
        self.ability = Ability(object["ability"])
        self.height_and_weight = HeightAndWeight(object["heightAndWeight"])
        self.age = Age(object["age"])
        self.darkvision = object["darkvision"] if "darkvision" in object.keys() else self.darkvision
        self.trait_tags = object["traitTags"] if "immune" in object.keys() else self.immune
        self.skill_proficiencies = SkillProficiencies(object["skillProficiencies"])
        self.language_proficiencies = LanguageProficiencies(object["languageProficiencies"])
        self.tool_proficiencies = ToolProficiencies(object["toolProficiencies"])
        self.weapon_proficiencies = WeaponProficiencies(object["weaponProficiencies"])
        self.armor_proficiencies = ArmorProficiencies(object["armorProficiencies"])
        self.resist = Resist(object["resist"])
        self.additional_spells = AdditionalSpells(object["additionalSpells"])
        self.immune = object["immune"] if "immune" in object.keys() else self.immune
        self.conditionImmune = object["conditionImmune"] if "immune" in object.keys() else self.immune
        self.entries = object["entries"] if "entries" in object.keys() else self.entries
        self.has_fluff = object["hasFluff"] if "hasFluff" in object.keys() else self.has_fluff
        self.has_fluff_images = object["hasFluffImages"] if "hasFluffImages" in object.keys() else self.has_fluff_images
        self.versions = object["_versions"] if "_versions" in object.keys() else self.versions


class Races:
    races: list[Race]

    def __init__(self) -> None:
        with open("./data/raw/races.json") as f:
            object = json.load(f)

            for race in object["race"]:
                self.races.append(Race(race))
                print(f"finished {self.races[-1].name} | {len(self.races)}/{len(object["race"])}")
