import json
import datetime
from typing import Any

from src.DAO.MiscClasses import (
    Ability,
    AdditionalSpells,
    ArmorProficiencies,
    Expertise,
    LanguageProficiencies,
    OptionalFeatureProgression,
    Prerequisite,
    SavingThrowProficiencies,
    SkillProficiencies,
    SkillToolLanguageProficiencies,
    ToolProficiencies,
    WeaponProficiencies,
)


class Feat:
    name: str
    source: str
    page: int
    srd: bool
    prerequisite: Prerequisite
    ability: Ability
    additional_spells: AdditionalSpells
    entries: list  # take from other DAO
    has_fluff_images: bool
    tool_proficiencies: ToolProficiencies
    optional_feature_progression: OptionalFeatureProgression
    resist: list[str]
    language_proficiencies: LanguageProficiencies
    weapon_proficiencies: WeaponProficiencies
    armor_proficiencies: ArmorProficiencies
    skill_proficiencies: SkillProficiencies
    saving_throw_proficiencies: SavingThrowProficiencies
    expertise: Expertise
    skill_tool_language_proficiencies: SkillToolLanguageProficiencies

    def __init__(self, object: dict[str, Any]) -> None:
        self.name = object["name"] if "name" in object.keys() else ""
        self.source = object["source"] if "source" in object.keys() else ""
        self.page = object["page"] if "page" in object.keys() else -1
        self.srd = object["srd"] if "srd" in object.keys() else False
        self.prerequisite = (
            Prerequisite(object["prerequisite"]) if "prerequisite" in object.keys() else Prerequisite(None)
        )
        self.ability = Ability(object["ability"]) if "ability" in object.keys() else Ability(None)
        self.additional_spells = (
            AdditionalSpells(object["additional_spells"])
            if "additional_spells" in object.keys()
            else AdditionalSpells(None)
        )
        self.entries = object["entries"] if "entries" in object.keys() else []
        self.has_fluff_images = object["has_fluff_images"] if "has_fluff_images" in object.keys() else False
        self.tool_proficiencies = (
            ToolProficiencies(object["tool_proficiencies"])
            if "tool_proficiencies" in object.keys()
            else ToolProficiencies(None)
        )
        self.optional_feature_progression = (
            OptionalFeatureProgression(object["optional_feature_progression"])
            if "optional_feature_progression" in object.keys()
            else OptionalFeatureProgression(None)
        )
        self.resist = object["resist"] if "resist" in object.keys() else []
        self.language_proficiencies = (
            LanguageProficiencies(object["language_proficiencies"])
            if "language_proficiencies" in object.keys()
            else LanguageProficiencies(None)
        )
        self.weapon_proficiencies = (
            WeaponProficiencies(object["weapon_proficiencies"])
            if "weapon_proficiencies" in object.keys()
            else WeaponProficiencies(None)
        )
        self.armor_proficiencies = (
            ArmorProficiencies(object["armor_proficiencies"])
            if "armor_proficiencies" in object.keys()
            else ArmorProficiencies(None)
        )
        self.skill_proficiencies = (
            SkillProficiencies(object["skill_proficiencies"])
            if "skill_proficiencies" in object.keys()
            else SkillProficiencies(None)
        )
        self.saving_throw_proficiencies = (
            SavingThrowProficiencies(object["saving_throw_proficiencies"])
            if "saving_throw_proficiencies" in object.keys()
            else SavingThrowProficiencies(None)
        )
        self.expertise = Expertise(object["expertise"]) if "expertise" in object.keys() else Expertise(None)
        self.skill_tool_language_proficiencies = (
            SkillToolLanguageProficiencies(object["skill_tool_language_proficiencies"])
            if "skill_tool_language_proficiencies" in object.keys()
            else SkillToolLanguageProficiencies(None)
        )

    def as_dict(self) -> dict[str, Any]:
        """This is effectivaly read only as it creates a copy

        Returns:
            dict[str, Any]: the class as a dict
        """

        copy = self.__dict__.copy()
        copy["prerequisite"] = self.prerequisite.as_dict()
        copy["ability"] = self.ability.__dict__
        copy["additional_spells"] = self.additional_spells.__dict__
        copy["tool_proficiencies"] = self.tool_proficiencies.__dict__
        copy["optional_feature_progression"] = self.optional_feature_progression.__dict__
        copy["language_proficiencies"] = self.language_proficiencies.__dict__
        copy["weapon_proficiencies"] = self.weapon_proficiencies.__dict__
        copy["armor_proficiencies"] = self.armor_proficiencies.__dict__
        copy["skill_proficiencies"] = self.skill_proficiencies.__dict__
        copy["saving_throw_proficiencies"] = self.saving_throw_proficiencies.__dict__
        copy["expertise"] = self.expertise.__dict__
        copy["skill_tool_language_proficiencies"] = self.skill_tool_language_proficiencies.as_dict()
        return copy


class Feats:
    feats: dict[str, Feat]

    def __init__(self) -> None:
        with open("data/raw/feats.json") as f:
            obj = json.load(f)

            self.feats = {}
            for feat in obj["feat"]:
                self.feats[feat["name"]] = Feat(feat)

    def as_dict(self) -> dict[str, Any]:
        """This is effectivaly read only as it creates a copy

        Returns:
            dict[str, Any]: the class as a dict
        """

        copy = self.__dict__.copy()
        for feat in self.feats.keys():
            copy["classes"][feat] = self.feats[feat].as_dict()
        return copy


if __name__ == "__main__":
    times = []
    count = 1000
    for _ in range(0, count):
        start = datetime.datetime.now()
        feats = Feats()
        end = datetime.datetime.now()
        times.append((end - start).microseconds)
        del feats

    average_time = sum(times) / count

    print(f"average time was {average_time} microseconds")
