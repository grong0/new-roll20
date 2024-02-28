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
        else:
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
    mature: int = None
    max: int = None

    def __init__(self, object: Optional[dict[str, int]]) -> None:
        if object is not None:
            self.mature = object["mature"] if "mature" in object.keys() else self.mature
            self.max = object["max"] if "max" in object.keys() else self.max


class LanguageProficiencies:
    languages: list[str] = []
    any_standard: int = 0
    choose_count: int = 0
    choose_languages: list[str]

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
                    self.choose_count = object["choose"]["count"]
                    self.choose_languages = object["choose"]["from"]
                else:
                    print(f"Language proficiency value was not expected")
                    print(f"It was a {type(object[key])} with a value of {object[key]}")


class Source:
    source: str = ""
    page: int = None

    def __init__(self, object: Optional[dict[str, str | int]] = None, source: str = None, page: int = None) -> None:
        if source is not None and page is not None:
            self.source = source
            self.page = page
        elif object is not None:
            self.source = object["source"]
            self.page = object["page"] if "page" in object.keys else self.page


class Race:
    name: str
    source: Source
    srd: bool
    basic_rules: bool
    other_sources: list[Source]
    reprinted_as
    copy
    lineage
    creature_types
    creature_type_tags
    size: list[str]
    speed: Speed
    ability: Ability
    height_and_weight
    age: Age
    darkvision: int
    trait_tags
    skill_proficiencies: SkillProficiencies
    language_proficiencies: LanguageProficiencies
    tool_proficiencies: ToolProficiencies
    weapon_proficiencies: WeaponProficiencies
    armor_proficiencies: ArmorProficiencies
    resist
    sound_clip
    additional_spells
    immune
    conditionImmune
    entries
    has_fluff: bool
    has_fluff_images: bool
    versions
